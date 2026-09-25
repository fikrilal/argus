use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use super::launcher::{AgentExecutionPlan, AgentRunResult, launch_agent};

pub const PERSONA_ARCHITECT_SYSTEM_PROMPT: &str = r#"You are the Argus Adversarial Persona Architect.
Your mission is to inspect the codebase in the current working directory and synthesize 3 to 5 specialized, adversarial QA personas tailored to this project's exact technology stack, architecture patterns, and critical business invariants.

## Inspection Strategy:
1. Identify primary language, frameworks, and build manifests:
   - Check manifests: `Cargo.toml`, `pubspec.yaml`, `package.json`, `go.mod`, `pyproject.toml`, `pom.xml`, `build.gradle`, etc.
2. Explore repository structure and architecture:
   - Check source trees: `src/`, `lib/`, `pkg/`, `internal/`, `app/`, etc.
   - Detect data persistence layers: SQLite, PostgreSQL, Drift, Room, Diesel, Prisma, Redis, etc.
   - Detect state management and concurrency: Bloc, Redux, Tokio, Goroutines, Rx, etc.
   - Detect external integrations: HTTP APIs, WebSockets, Bluetooth, Sensors, Payment gateways, etc.
3. Identify critical business invariants and failure modes:
   - What breaks if the network drops or requests retry?
   - What breaks if database transactions fail or rollback?
   - What boundary validation flaws could corrupt state?
   - Where could deadlocks, race conditions, memory leaks, or unhandled panics occur?

## Persona Output Requirements:
For each tailored persona you create, you MUST write a Markdown file directly into `.argus/personas/<persona-name>.md`.
Each persona file MUST strictly follow the standard Argus persona schema:

---
name: <kebab-case-name>
title: <Descriptive Title>
squad: <squad-name>
model_tier: standard
tools: read, grep, find, ls, bash
---

<System prompt explaining who the persona is, what they attack, failure scenarios to hunt for, and the standard Argus reporting rubric.>

## Rules for Personas:
- Do NOT create generic code-style reviewers (e.g. "general-clean-code-reviewer").
- Make every persona an aggressive, domain-specific adversarial saboteur focused on real bugs in this codebase (e.g. `sqlite-outbox-auditor`, `jwt-refresh-adversary`, `tokio-concurrency-auditor`).
- Each persona MUST include the standard Argus Reporting Rubric:
  - **Status:** [VIOLATION / PASS]
  - **Target:** `<file_path>:<line_number>`
  - **Issue:** Summary of what is broken or missing
  - **Failure Scenario:** How a real user, network drop, or human QA would trigger this bug
  - **Recommended Fix:** Concrete, idiomatic code snippet to resolve the issue
- Do NOT invoke recursive audit commands (`argus audit` or `cargo run -- audit`).
"#;

pub const PERSONA_ARCHITECT_TASK_PAYLOAD: &str = r"Inspect the codebase in the current working directory. Detect the technology stack, architecture patterns, and high-risk boundaries. Synthesize 3 to 5 tailored adversarial persona files directly into `.argus/personas/`. Ensure all persona files have valid frontmatter (`name`, `title`, `squad`, `model_tier`, `tools`) and detailed adversarial attack instructions.";

/// Launches the Persona Architect subagent to inspect the codebase and synthesize tailored personas.
///
/// Returns the list of created persona files in `<target_dir>/.argus/personas/`.
pub async fn bootstrap_tailored_personas(
    target_dir: &Path,
    pi_bin_override: Option<&str>,
) -> Result<Vec<PathBuf>> {
    let personas_dir = target_dir.join(".argus").join("personas");
    if !personas_dir.exists() {
        fs::create_dir_all(&personas_dir).with_context(|| {
            format!("Failed to create directory at '{}'", personas_dir.display())
        })?;
    }

    // Record existing personas before launching
    let mut existing_files = Vec::new();
    if personas_dir.is_dir()
        && let Ok(entries) = fs::read_dir(&personas_dir)
    {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "md") {
                existing_files.push(path);
            }
        }
    }

    let plan = AgentExecutionPlan {
        persona_name: "persona-architect".to_string(),
        session_name: "argus/bootstrap/persona-architect".to_string(),
        system_prompt: PERSONA_ARCHITECT_SYSTEM_PROMPT.to_string(),
        task_payload: PERSONA_ARCHITECT_TASK_PAYLOAD.to_string(),
        tools: vec![
            "read".to_string(),
            "grep".to_string(),
            "find".to_string(),
            "ls".to_string(),
            "bash".to_string(),
            "write".to_string(),
            "edit".to_string(),
        ],
        model: None,
    };

    let result: AgentRunResult = launch_agent(&plan, pi_bin_override, target_dir).await?;

    if result.exit_code != 0 && result.stdout.trim().is_empty() {
        let err_msg = if result.stderr.trim().is_empty() {
            format!("Process exited with status code {}", result.exit_code)
        } else {
            result.stderr.trim().to_string()
        };
        anyhow::bail!("Persona architect subagent failed: {err_msg}");
    }

    // Collect newly generated persona files
    let mut generated_files = Vec::new();
    if personas_dir.is_dir()
        && let Ok(entries) = fs::read_dir(&personas_dir)
    {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "md") && !existing_files.contains(&path) {
                generated_files.push(path);
            }
        }
    }

    generated_files.sort();
    Ok(generated_files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_architect_prompts_contain_required_instructions() {
        assert!(PERSONA_ARCHITECT_SYSTEM_PROMPT.contains("Argus Adversarial Persona Architect"));
        assert!(PERSONA_ARCHITECT_SYSTEM_PROMPT.contains(".argus/personas/<persona-name>.md"));
        assert!(PERSONA_ARCHITECT_SYSTEM_PROMPT.contains("Reporting Rubric"));
        assert!(PERSONA_ARCHITECT_TASK_PAYLOAD.contains(".argus/personas/"));
    }
}
