use super::sfd::SfdDocument;
use std::fmt::Write;

/// Bundles the operational context needed to instruct an Argus subagent session.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskContextBundle {
    /// Active Git branch name
    pub branch: String,
    /// Optional base branch to compare against (e.g. `origin/main`)
    pub base_branch: Option<String>,
    /// Whether only staged changes should be audited
    pub staged_only: bool,
    /// Active SFD specification document (if present)
    pub sfd: Option<SfdDocument>,
}

impl TaskContextBundle {
    /// Creates a new `TaskContextBundle`.
    pub fn new(
        branch: String,
        base_branch: Option<String>,
        staged_only: bool,
        sfd: Option<SfdDocument>,
    ) -> Self {
        Self {
            branch,
            base_branch,
            staged_only,
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
    prompt.push_str("Focus exclusively on auditing the code modifications in this repository. Do NOT search outside the current working directory.\n");
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
        let bundle = TaskContextBundle::new("main".to_string(), None, false, None);

        let prompt = build_agent_prompt(&bundle, "form-boundary-saboteur");
        assert!(prompt.contains("'form-boundary-saboteur'"));
        assert!(prompt.contains("Git branch: 'main'"));
        assert!(prompt.contains("git diff HEAD"));
        assert!(!prompt.contains("Specification Reference"));
        assert!(prompt.contains("### 3. Reporting Rubric"));
    }

    #[test]
    fn test_build_agent_prompt_staged_only() {
        let bundle = TaskContextBundle::new("DEV-AFM-AUTH_MIGRATION".to_string(), None, true, None);

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

        let bundle = TaskContextBundle::new("feature-kulakan".to_string(), None, false, Some(sfd));

        let prompt = build_agent_prompt(&bundle, "sfd-clause-detective");
        assert!(prompt.contains("Specification Reference (Ground Truth)"));
        assert!(prompt.contains("docs/sfd/kulakan.md"));
        assert!(prompt.contains("Kulakan Spec"));
    }
}
