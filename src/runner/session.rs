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
