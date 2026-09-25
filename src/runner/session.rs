use anyhow::{Context, Result};
use std::path::Path;
use std::process::ExitStatus;
use tokio::process::Command;

/// Generates a deterministic Pi session name for an Argus agent run.
///
/// Convention:
/// `argus/<branch-slug>/<persona-slug>`
///
/// Example:
/// `argus/DEV-AFM-AUTH_MIGRATION/stock-ledger-auditor`
#[allow(dead_code)]
pub fn format_session_name(branch_slug: &str, persona_slug: &str) -> String {
    let clean_branch = if branch_slug.trim().is_empty() {
        "main"
    } else {
        branch_slug.trim()
    };

    let clean_persona = if persona_slug.trim().is_empty() {
        "agent"
    } else {
        persona_slug.trim()
    };

    format!("argus/{clean_branch}/{clean_persona}")
}

/// Launches an interactive `pi --resume <session_name>` subprocess handing stdio over to the user.
pub async fn resume_interactive_session(
    session_name: &str,
    current_dir: &Path,
) -> Result<ExitStatus> {
    let pi_bin = std::env::var("ARGUS_PI_BIN").unwrap_or_else(|_| "pi".to_string());

    let mut cmd = Command::new(&pi_bin);
    cmd.current_dir(current_dir);
    cmd.arg("--resume").arg(session_name);

    let status = match cmd.status().await {
        Ok(s) => s,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            anyhow::bail!(
                "The '{pi_bin}' binary was not found on PATH. Please ensure Pi (https://pi.dev) is installed and available in your environment."
            );
        }
        Err(e) => {
            return Err(e)
                .with_context(|| format!("Failed to spawn '{pi_bin} --resume {session_name}'"));
        }
    };

    Ok(status)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_session_name() {
        assert_eq!(
            format_session_name("DEV-AFM-AUTH_MIGRATION", "stock-ledger-auditor"),
            "argus/DEV-AFM-AUTH_MIGRATION/stock-ledger-auditor"
        );
        assert_eq!(
            format_session_name("feature-kulakan", "sfd-clause-detective"),
            "argus/feature-kulakan/sfd-clause-detective"
        );
    }

    #[test]
    fn test_format_session_name_fallbacks() {
        assert_eq!(
            format_session_name("", "stock-ledger-auditor"),
            "argus/main/stock-ledger-auditor"
        );
        assert_eq!(
            format_session_name("my-branch", ""),
            "argus/my-branch/agent"
        );
    }
}
