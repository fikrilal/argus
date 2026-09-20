use anyhow::{Context, Result};
use std::path::Path;
use tokio::process::Command;

/// Asynchronously detects the active Git branch in the specified repository directory.
///
/// If the repository is in a detached `HEAD` state, returns `detached-<short_hash>`.
pub async fn detect_current_branch(repo_root: &Path) -> Result<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .await
        .with_context(|| {
            format!(
                "Failed to execute git command in directory '{}'",
                repo_root.display()
            )
        })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!(
            "Git branch detection failed in '{}': {}",
            repo_root.display(),
            stderr.trim()
        );
    }

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();

    if branch == "HEAD" {
        // Detached HEAD state: obtain short commit hash
        let sha_output = Command::new("git")
            .arg("-C")
            .arg(repo_root)
            .arg("rev-parse")
            .arg("--short")
            .arg("HEAD")
            .output()
            .await
            .with_context(|| {
                format!(
                    "Failed to get short commit hash in '{}'",
                    repo_root.display()
                )
            })?;

        if sha_output.status.success() {
            let sha = String::from_utf8_lossy(&sha_output.stdout)
                .trim()
                .to_string();
            return Ok(format!("detached-{sha}"));
        }

        return Ok("detached".to_string());
    }

    Ok(branch)
}

/// Converts a Git branch name into a clean, safe slug for session naming and filesystem paths.
///
/// Rules:
/// - Replaces slashes (`/`, `\`), spaces, and unsafe punctuation (`#`, `@`, `:`, etc.) with hyphens (`-`).
/// - Retains alphanumeric characters, hyphens (`-`), underscores (`_`), and dots (`.`).
/// - Collapses multiple consecutive hyphens into a single hyphen.
/// - Trims leading and trailing hyphens.
/// - Falls back to `"main"` if the input becomes empty.
pub fn slugify_branch_name(branch: &str) -> String {
    let mut slug = String::with_capacity(branch.len());
    let mut last_was_dash = false;

    for ch in branch.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' || ch == '.' {
            slug.push(ch);
            last_was_dash = false;
        } else if !last_was_dash {
            slug.push('-');
            last_was_dash = true;
        }
    }

    let trimmed = slug.trim_matches('-');

    if trimmed.is_empty() {
        "main".to_string()
    } else {
        trimmed.to_string()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn test_slugify_branch_name_with_slashes() {
        assert_eq!(
            slugify_branch_name("DEV/AFM/AUTH_MIGRATION"),
            "DEV-AFM-AUTH_MIGRATION"
        );
        assert_eq!(
            slugify_branch_name("feature/stockist-kulakan-v2"),
            "feature-stockist-kulakan-v2"
        );
        assert_eq!(
            slugify_branch_name("bugfix/issue-123/sub-fix"),
            "bugfix-issue-123-sub-fix"
        );
    }

    #[test]
    fn test_slugify_branch_name_with_special_characters() {
        assert_eq!(
            slugify_branch_name("feature/fix#123@client:test"),
            "feature-fix-123-client-test"
        );
        assert_eq!(
            slugify_branch_name("user/john doe/my branch"),
            "user-john-doe-my-branch"
        );
    }

    #[test]
    fn test_slugify_branch_name_collapses_dashes() {
        assert_eq!(slugify_branch_name("a///b---c"), "a-b-c");
        assert_eq!(slugify_branch_name("---branch---"), "branch");
    }

    #[test]
    fn test_slugify_branch_name_fallback_on_empty() {
        assert_eq!(slugify_branch_name(""), "main");
        assert_eq!(slugify_branch_name("///"), "main");
        assert_eq!(slugify_branch_name("---"), "main");
    }

    #[tokio::test]
    async fn test_detect_current_branch_in_current_repo() {
        let repo_root = Path::new(".");
        let branch = detect_current_branch(repo_root)
            .await
            .expect("should detect branch in current git repo");
        assert_eq!(branch, "main");
    }
}
