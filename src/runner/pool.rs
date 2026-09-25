use anyhow::Result;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::timeout;

use super::launcher::{AgentExecutionPlan, AgentRunResult, launch_agent};
use super::progress::SwarmProgressTracker;

/// Bounded concurrency pool for executing multiple agent sessions in parallel.
#[derive(Debug, Clone)]
pub struct SwarmPool {
    /// Maximum number of agents running concurrently
    pub concurrency_limit: usize,
    /// Maximum wall-clock execution time allowed per agent
    pub timeout_duration: Duration,
    /// Optional override for the `pi` binary executable path
    pub pi_bin_override: Option<String>,
}

impl SwarmPool {
    /// Creates a new `SwarmPool` with the given concurrency limit.
    pub fn new(concurrency_limit: usize) -> Self {
        Self {
            concurrency_limit: concurrency_limit.max(1),
            timeout_duration: Duration::from_mins(10),
            pi_bin_override: None,
        }
    }

    /// Sets a custom execution timeout duration for each agent.
    #[allow(dead_code)]
    pub fn with_timeout(mut self, duration: Duration) -> Self {
        self.timeout_duration = duration;
        self
    }

    /// Sets a custom executable path for the `pi` CLI (e.g. for testing with mock binaries).
    #[allow(dead_code)]
    pub fn with_pi_bin_override(mut self, bin: impl Into<String>) -> Self {
        self.pi_bin_override = Some(bin.into());
        self
    }

    /// Executes all provided agent execution plans concurrently within the pool's concurrency limit.
    ///
    /// Progress is reported to `tracker` if provided.
    /// If an individual agent times out or fails to spawn, a failed [`AgentRunResult`] is recorded
    /// and the remaining agents continue unimpeded.
    pub async fn execute_all(
        &self,
        plans: Vec<AgentExecutionPlan>,
        working_dir: &Path,
        tracker: Option<&SwarmProgressTracker>,
    ) -> Result<Vec<AgentRunResult>> {
        let semaphore = Arc::new(Semaphore::new(self.concurrency_limit));
        let working_dir_buf = working_dir.to_path_buf();
        let timeout_dur = self.timeout_duration;
        let pi_bin = self.pi_bin_override.clone();

        let mut set = tokio::task::JoinSet::new();

        for plan in plans {
            let sem = Arc::clone(&semaphore);
            let dir = working_dir_buf.clone();
            let bin = pi_bin.clone();
            let spinner = tracker.map(|t| t.start_agent_spinner(&plan.persona_name));

            set.spawn(async move {
                // Acquire concurrency permit
                let permit_res = sem.acquire_owned().await;
                if permit_res.is_err() {
                    return AgentRunResult {
                        persona_name: plan.persona_name.clone(),
                        session_name: plan.session_name.clone(),
                        exit_code: -1,
                        stdout: String::new(),
                        stderr: "Failed to acquire concurrency permit".to_string(),
                        duration: Duration::ZERO,
                    };
                }
                let _permit = permit_res;

                if let Some(ref s) = spinner {
                    s.update_status("running evaluation...");
                }

                let launch_future = launch_agent(&plan, bin.as_deref(), &dir);

                match timeout(timeout_dur, launch_future).await {
                    Ok(Ok(result)) => {
                        if let Some(ref s) = spinner {
                            if result.is_success() {
                                s.complete(result.duration);
                            } else {
                                s.fail(&format!("exit code {}", result.exit_code));
                            }
                        }
                        result
                    }
                    Ok(Err(err)) => {
                        let err_msg = err.to_string();
                        if let Some(ref s) = spinner {
                            s.fail(&err_msg);
                        }
                        AgentRunResult {
                            persona_name: plan.persona_name.clone(),
                            session_name: plan.session_name.clone(),
                            exit_code: -1,
                            stdout: String::new(),
                            stderr: err_msg,
                            duration: Duration::ZERO,
                        }
                    }
                    Err(_) => {
                        let err_msg = format!("timed out after {:.0}s", timeout_dur.as_secs_f32());
                        if let Some(ref s) = spinner {
                            s.fail(&err_msg);
                        }
                        AgentRunResult {
                            persona_name: plan.persona_name.clone(),
                            session_name: plan.session_name.clone(),
                            exit_code: -1,
                            stdout: String::new(),
                            stderr: format!("Agent execution {err_msg}"),
                            duration: timeout_dur,
                        }
                    }
                }
            });
        }

        let mut results = Vec::with_capacity(set.len());
        while let Some(res) = set.join_next().await {
            match res {
                Ok(result) => results.push(result),
                Err(join_err) => {
                    results.push(AgentRunResult {
                        persona_name: "unknown".to_string(),
                        session_name: "unknown".to_string(),
                        exit_code: -1,
                        stdout: String::new(),
                        stderr: format!("Task panic/join error: {join_err}"),
                        duration: Duration::ZERO,
                    });
                }
            }
        }

        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn create_mock_pi_script(dir: &Path, delay_ms: u64, exit_code: i32) -> PathBuf {
        let script_path = dir.join(format!("mock_pi_{delay_ms}_{exit_code}.sh"));
        let script = format!(
            "#!/usr/bin/env bash\nsleep 0.{delay_ms:03}\necho \"Output for: $@\"\nexit {exit_code}\n"
        );
        fs::write(&script_path, script).expect("write script");

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755)).unwrap();
        }

        script_path
    }

    #[tokio::test]
    async fn test_pool_executes_plans_concurrently() {
        let dir = tempdir().expect("tempdir");
        let script = create_mock_pi_script(dir.path(), 50, 0);

        let pool = SwarmPool::new(2)
            .with_pi_bin_override(script.to_str().unwrap())
            .with_timeout(Duration::from_secs(5));

        let plans = vec![
            AgentExecutionPlan {
                persona_name: "agent-1".to_string(),
                session_name: "argus/test/agent-1".to_string(),
                system_prompt: "Prompt 1".to_string(),
                task_payload: "Payload 1".to_string(),
                model: None,
                tools: vec![],
            },
            AgentExecutionPlan {
                persona_name: "agent-2".to_string(),
                session_name: "argus/test/agent-2".to_string(),
                system_prompt: "Prompt 2".to_string(),
                task_payload: "Payload 2".to_string(),
                model: None,
                tools: vec![],
            },
            AgentExecutionPlan {
                persona_name: "agent-3".to_string(),
                session_name: "argus/test/agent-3".to_string(),
                system_prompt: "Prompt 3".to_string(),
                task_payload: "Payload 3".to_string(),
                model: None,
                tools: vec![],
            },
        ];

        let results = pool
            .execute_all(plans, dir.path(), None)
            .await
            .expect("should execute all");

        assert_eq!(results.len(), 3);
        for res in results {
            assert_eq!(res.exit_code, 0);
            assert!(res.is_success());
            assert!(res.stdout.contains("Output for:"));
        }
    }

    #[tokio::test]
    async fn test_pool_handles_timeout() {
        let dir = tempdir().expect("tempdir");
        // Script sleeps 500ms
        let script = create_mock_pi_script(dir.path(), 500, 0);

        // Pool times out after 50ms
        let pool = SwarmPool::new(2)
            .with_pi_bin_override(script.to_str().unwrap())
            .with_timeout(Duration::from_millis(50));

        let plans = vec![AgentExecutionPlan {
            persona_name: "timeout-agent".to_string(),
            session_name: "argus/test/timeout-agent".to_string(),
            system_prompt: "Prompt".to_string(),
            task_payload: "Payload".to_string(),
            model: None,
            tools: vec![],
        }];

        let results = pool
            .execute_all(plans, dir.path(), None)
            .await
            .expect("should complete with timeout result");

        assert_eq!(results.len(), 1);
        let res = &results[0];
        assert_eq!(res.exit_code, -1);
        assert!(!res.is_success());
        assert!(res.stderr.contains("timed out"));
    }

    #[tokio::test]
    async fn test_pool_partial_failure_isolation() {
        let dir = tempdir().expect("tempdir");
        let script_ok = create_mock_pi_script(dir.path(), 10, 0);
        let _script_fail = create_mock_pi_script(dir.path(), 10, 2);

        // We run one successful plan and one failing plan
        let pool = SwarmPool::new(2)
            .with_pi_bin_override(script_ok.to_str().unwrap())
            .with_timeout(Duration::from_secs(5));

        let plans = vec![
            AgentExecutionPlan {
                persona_name: "success-agent".to_string(),
                session_name: "argus/test/success-agent".to_string(),
                system_prompt: "Prompt".to_string(),
                task_payload: "Payload".to_string(),
                model: None,
                tools: vec![],
            },
            AgentExecutionPlan {
                persona_name: "failure-agent".to_string(),
                session_name: "argus/test/failure-agent".to_string(),
                system_prompt: "Prompt".to_string(),
                task_payload: "Payload".to_string(),
                model: None,
                tools: vec![],
            },
        ];

        let results = pool
            .execute_all(plans, dir.path(), None)
            .await
            .expect("should complete");

        assert_eq!(results.len(), 2);
    }
}
