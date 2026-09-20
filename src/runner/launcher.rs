use anyhow::{Context, Result};
use std::io::Write;
use std::path::Path;
use std::time::{Duration, Instant};
use tempfile::NamedTempFile;
use tokio::process::Command;

/// Execution specifications for launching a single agent session.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub struct AgentExecutionPlan {
    /// Name of the persona being executed
    pub persona_name: String,
    /// Deterministic session name (e.g. `argus/<branch>/<persona>`)
    pub session_name: String,
    /// Persona system prompt instructions
    pub system_prompt: String,
    /// Prompt task payload passed to the agent
    pub task_payload: String,
    /// Optional model override (e.g. `anthropic/claude-3-7-sonnet:high`)
    pub model: Option<String>,
    /// Allowed tools list (e.g. `read, grep, find, ls, bash`)
    pub tools: Vec<String>,
}

/// The result returned from an agent's execution run.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub struct AgentRunResult {
    /// Name of the persona executed
    pub persona_name: String,
    /// Session name identifier
    pub session_name: String,
    /// Subprocess exit code (0 for success)
    pub exit_code: i32,
    /// Captured standard output
    pub stdout: String,
    /// Captured standard error
    pub stderr: String,
    /// Wall-clock execution duration
    pub duration: Duration,
}

impl AgentRunResult {
    /// Returns true if the agent process exited with status code 0.
    #[allow(dead_code)]
    pub fn is_success(&self) -> bool {
        self.exit_code == 0
    }
}

/// Launches a single agent subprocess asynchronously via the `pi` CLI.
///
/// Uses non-interactive mode (`pi -p`), setting the session name, appending the persona's
/// system prompt via a temporary file, and passing the evaluation prompt.
#[allow(dead_code)]
pub async fn launch_agent(
    plan: &AgentExecutionPlan,
    pi_bin_override: Option<&str>,
    working_dir: &Path,
) -> Result<AgentRunResult> {
    let pi_bin = std::env::var("ARGUS_PI_BIN")
        .ok()
        .or_else(|| pi_bin_override.map(ToString::to_string))
        .unwrap_or_else(|| "pi".to_string());

    // Write system prompt to a temporary file so Pi can load it via --append-system-prompt
    let mut prompt_file = NamedTempFile::new()
        .with_context(|| "Failed to create temporary file for persona prompt")?;
    prompt_file
        .write_all(plan.system_prompt.as_bytes())
        .with_context(|| "Failed to write persona prompt to temporary file")?;
    prompt_file.flush()?;

    let prompt_file_path = prompt_file.path().to_string_lossy().to_string();

    let mut cmd = Command::new(&pi_bin);
    cmd.current_dir(working_dir);
    cmd.arg("-p"); // Non-interactive mode
    cmd.arg("--name").arg(&plan.session_name);
    cmd.arg("--append-system-prompt").arg(&prompt_file_path);

    if !plan.tools.is_empty() {
        cmd.arg("--tools").arg(plan.tools.join(","));
    }

    if let Some(ref model) = plan.model {
        cmd.arg("--model").arg(model);
    }

    // The user task message
    cmd.arg(&plan.task_payload);

    let start_time = Instant::now();

    let output = match cmd.output().await {
        Ok(out) => out,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            anyhow::bail!(
                "The '{pi_bin}' binary was not found on PATH. Please ensure Pi (https://pi.dev) is installed and available in your environment."
            );
        }
        Err(e) => {
            return Err(e).with_context(|| format!("Failed to spawn agent process '{pi_bin}'"));
        }
    };

    let duration = start_time.elapsed();
    let exit_code = output.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    Ok(AgentRunResult {
        persona_name: plan.persona_name.clone(),
        session_name: plan.session_name.clone(),
        exit_code,
        stdout,
        stderr,
        duration,
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_launch_agent_with_mock_binary() {
        let dir = tempdir().expect("tempdir");

        // Create a mock executable script mimicking `pi`
        let mock_script_path = dir.path().join("mock_pi.sh");
        let script = "#!/usr/bin/env bash\necho \"Mock agent executed for: $@\"\nexit 0\n";
        {
            use std::io::Write;
            let mut f = fs::File::create(&mock_script_path).unwrap();
            f.write_all(script.as_bytes()).unwrap();
            f.sync_all().unwrap();
            drop(f);
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&mock_script_path, fs::Permissions::from_mode(0o755)).unwrap();
        }

        tokio::time::sleep(std::time::Duration::from_millis(20)).await;

        let plan = AgentExecutionPlan {
            persona_name: "test-saboteur".to_string(),
            session_name: "argus/test/test-saboteur".to_string(),
            system_prompt: "Attack all boundaries.".to_string(),
            task_payload: "Audit git diff".to_string(),
            model: Some("test-model".to_string()),
            tools: vec!["read".to_string(), "bash".to_string()],
        };

        let result = launch_agent(&plan, Some(mock_script_path.to_str().unwrap()), dir.path())
            .await
            .expect("should launch successfully");

        assert_eq!(result.persona_name, "test-saboteur");
        assert_eq!(result.session_name, "argus/test/test-saboteur");
        assert_eq!(result.exit_code, 0);
        assert!(result.is_success());
        assert!(result.stdout.contains("Mock agent executed for:"));
        assert!(result.stdout.contains("--name argus/test/test-saboteur"));
    }

    #[tokio::test]
    async fn test_launch_agent_missing_binary_returns_helpful_error() {
        let dir = tempdir().expect("tempdir");
        let plan = AgentExecutionPlan {
            persona_name: "test-saboteur".to_string(),
            session_name: "argus/test/test-saboteur".to_string(),
            system_prompt: "System prompt".to_string(),
            task_payload: "Payload".to_string(),
            model: None,
            tools: vec![],
        };

        let result = launch_agent(&plan, Some("non_existent_binary_xyz_123"), dir.path()).await;

        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("binary was not found on PATH"));
        assert!(err_msg.contains("https://pi.dev"));
    }
}
