use super::sfd::SfdDocument;
use std::fmt::Write;
use std::path::PathBuf;

/// Bundles the operational context needed to instruct an Argus subagent session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskContextBundle {
    /// Active Git branch name
    pub branch: String,
    /// Optional base branch to compare against (e.g. `origin/main`)
    pub base_branch: Option<String>,
    /// Whether only staged changes should be audited
    pub staged_only: bool,
    /// Whether the entire codebase should be audited instead of diff changes
    pub full_codebase: bool,
    /// Optional specific feature directory or file path to target
    pub target_path: Option<PathBuf>,
    /// Active SFD specification document (if present)
    pub sfd: Option<SfdDocument>,
}

impl TaskContextBundle {
    /// Creates a new `TaskContextBundle`.
    pub fn new(
        branch: String,
        base_branch: Option<String>,
        staged_only: bool,
        full_codebase: bool,
        target_path: Option<PathBuf>,
        sfd: Option<SfdDocument>,
    ) -> Self {
        Self {
            branch,
            base_branch,
            staged_only,
            full_codebase,
            target_path,
            sfd,
        }
    }
}

/// Constructs the initial prompt payload passed to an Argus subagent when launched.
#[allow(dead_code)]
pub fn build_agent_prompt(bundle: &TaskContextBundle, persona_name: &str) -> String {
    let mut prompt = String::new();

    let _ = writeln!(
        prompt,
        "You are deployed as the Argus adversarial QA persona: '{persona_name}'."
    );
    let _ = writeln!(
        prompt,
        "Your mission is to evaluate code changes on Git branch: '{}'.\n",
        bundle.branch
    );

    prompt.push_str("### 1. Code Inspection Instructions\n");
    if let Some(ref target) = bundle.target_path {
        let _ = writeln!(
            prompt,
            "You are auditing the specific target subsystem directory: '{}'",
            target.display()
        );
        prompt.push_str("Focus your inspection strictly on the files within this target path and their related contracts.\n\n");
    } else if bundle.full_codebase {
        prompt.push_str("You are auditing the ENTIRE codebase in this repository (full-project review mode, not limited to a diff).\n");
        prompt.push_str("Inspect the source files under `src/`, `tests/`, and project configuration files using your `read`, `find`, and `grep` tools.\n");
        prompt.push_str("Focus on cross-module architecture, invariants, edge cases, error handling, and code quality across the entire project.\n\n");
    } else {
        prompt.push_str("Focus exclusively on auditing the code modifications in this repository. Do NOT search outside the current working directory.\n");
        prompt.push_str("Do NOT run broad repository-wide grep searches or inspect unrelated files. Limit your inspection strictly to the files modified in the diff.\n");
        if bundle.staged_only {
            prompt.push_str("Inspect the currently staged changes using your bash tool:\n");
            prompt.push_str("  `git diff --staged`\n");
        } else if let Some(ref base) = bundle.base_branch {
            let _ = writeln!(
                prompt,
                "Inspect the changes on this branch compared to the base branch using your bash tool:\n  `git diff {base}...HEAD`"
            );
        } else {
            prompt.push_str(
                "Inspect all uncommitted working tree changes using your bash tool:\n  `git status`\n  `git diff HEAD`\n",
            );
        }
        prompt.push_str("Use your `read` tool to inspect full file contents around any changed areas for necessary context.\n\n");
    }

    if let Some(ref sfd) = bundle.sfd {
        prompt.push_str("### 2. Specification Reference (Ground Truth)\n");
        let _ = writeln!(
            prompt,
            "The active System Functional Design (SFD) specification is available at: '{}'",
            sfd.path.display()
        );
        if let Some(ref title) = sfd.title {
            let _ = writeln!(prompt, "Document Title: '{title}'");
        }
        prompt.push_str("Use your `read` tool to inspect the business rules, acceptance criteria, and state transitions in this SFD.\n\n");
    }

    prompt.push_str("### 3. Reporting Rubric\n");
    prompt.push_str("Do NOT invoke recursive audit commands (such as `argus audit` or `cargo run -- audit`). Use static code inspection, `--help`, or `cargo test` to verify behavior.\n");
    prompt.push_str("Audit the code strictly according to your persona's system prompt.\n");
    prompt.push_str("If you identify defects, boundary oversights, missing rollbacks, or spec violations, report each finding in this exact format:\n\n");
    prompt.push_str("- **Status:** [VIOLATION / PASS]\n");
    prompt.push_str("- **Target:** `<file_path>:<line_number>`\n");
    prompt.push_str("- **Issue:** Concise summary of what is broken or missing\n");
    prompt.push_str(
        "- **Failure Scenario:** How a user, network drop, or human QA would trigger this bug\n",
    );
    prompt.push_str(
        "- **Recommended Fix:** Concrete, idiomatic code snippet to resolve the issue\n\n",
    );
    prompt.push_str("If no violations are found under your persona's scope, clearly state `[STATUS: PASS]` with a 1-sentence summary.\n");

    prompt
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_build_agent_prompt_working_tree_no_sfd() {
        let bundle = TaskContextBundle::new("main".to_string(), None, false, false, None, None);

        let prompt = build_agent_prompt(&bundle, "form-boundary-saboteur");
        assert!(prompt.contains("'form-boundary-saboteur'"));
        assert!(prompt.contains("Git branch: 'main'"));
        assert!(prompt.contains("git diff HEAD"));
        assert!(!prompt.contains("Specification Reference"));
        assert!(prompt.contains("### 3. Reporting Rubric"));
    }

    #[test]
    fn test_build_agent_prompt_staged_only() {
        let bundle = TaskContextBundle::new(
            "DEV-AFM-AUTH_MIGRATION".to_string(),
            None,
            true,
            false,
            None,
            None,
        );

        let prompt = build_agent_prompt(&bundle, "stock-ledger-auditor");
        assert!(prompt.contains("'stock-ledger-auditor'"));
        assert!(prompt.contains("git diff --staged"));
        assert!(!prompt.contains("git diff HEAD"));
    }

    #[test]
    fn test_build_agent_prompt_with_base_branch() {
        let bundle = TaskContextBundle::new(
            "feature-kulakan".to_string(),
            Some("origin/main".to_string()),
            false,
            false,
            None,
            None,
        );

        let prompt = build_agent_prompt(&bundle, "payload-pessimist");
        assert!(prompt.contains("git diff origin/main...HEAD"));
    }

    #[test]
    fn test_build_agent_prompt_with_sfd() {
        let sfd = SfdDocument::new(
            PathBuf::from("docs/sfd/kulakan.md"),
            "# Kulakan Spec\n\nRules here".to_string(),
        );

        let bundle = TaskContextBundle::new(
            "feature-kulakan".to_string(),
            None,
            false,
            false,
            None,
            Some(sfd),
        );

        let prompt = build_agent_prompt(&bundle, "sfd-clause-detective");
        assert!(prompt.contains("Specification Reference (Ground Truth)"));
        assert!(prompt.contains("docs/sfd/kulakan.md"));
        assert!(prompt.contains("Kulakan Spec"));
    }

    #[test]
    fn test_build_agent_prompt_full_codebase() {
        let bundle = TaskContextBundle::new("main".to_string(), None, false, true, None, None);

        let prompt = build_agent_prompt(&bundle, "rust-safety-auditor");
        assert!(prompt.contains("auditing the ENTIRE codebase"));
        assert!(prompt.contains("under `src/`, `tests/`"));
        assert!(!prompt.contains("git diff"));
    }

    #[test]
    fn test_build_agent_prompt_target_path() {
        let bundle = TaskContextBundle::new(
            "main".to_string(),
            None,
            false,
            false,
            Some(PathBuf::from("src/runner")),
            None,
        );

        let prompt = build_agent_prompt(&bundle, "tokio-concurrency-auditor");
        assert!(prompt.contains("target subsystem directory: 'src/runner'"));
    }
}
