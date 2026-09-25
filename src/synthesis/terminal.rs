use colored::Colorize;
use std::fmt::Write;

use super::synthesizer::AuditSynthesis;

/// Formats and renders an [`AuditSynthesis`] into a colorized terminal dashboard string.
#[must_use]
pub fn render_terminal_dashboard(synthesis: &AuditSynthesis) -> String {
    let mut out = String::new();

    let divider = "═".repeat(64);

    let _ = writeln!(out, "\n{}", divider.dimmed());

    if synthesis.is_passed() {
        let _ = writeln!(
            out,
            " {} {}",
            "✔".bold().green(),
            "ARGUS AUDIT PASSED: All agent checks satisfied!"
                .bold()
                .green()
        );
        let _ = writeln!(
            out,
            " Zero critical defects identified. Ready for pull request & QA handoff!"
        );
        if !synthesis.passed_personas.is_empty() {
            let clean_names = synthesis.passed_personas.join(", ");
            let _ = writeln!(
                out,
                " {} Clean Personas: ({clean_names})",
                "✔".bold().green()
            );
        }
    } else {
        let _ = writeln!(
            out,
            " {} {} ({} blocker, {} major, {} polish)",
            "✖".bold().red(),
            "ARGUS AUDIT ACTION REQUIRED: Defects identified"
                .bold()
                .red(),
            synthesis.blocker_count.to_string().bold().red(),
            synthesis.major_count.to_string().bold().yellow(),
            synthesis.polish_count.to_string().bold().cyan(),
        );

        for (idx, finding) in synthesis.findings.iter().enumerate() {
            let num = idx + 1;
            let _ = writeln!(out);
            let _ = writeln!(
                out,
                "{num}. {} {}",
                finding.severity.badge(),
                finding.title.bold()
            );
            let _ = writeln!(
                out,
                "   {:<10} {}",
                "Target:".dimmed(),
                finding.target_display().yellow()
            );
            let _ = writeln!(
                out,
                "   {:<10} {}",
                "Persona:".dimmed(),
                finding.persona.cyan()
            );
        }

        if !synthesis.passed_personas.is_empty() {
            let _ = writeln!(out);
            let clean_names = synthesis.passed_personas.join(", ");
            let _ = writeln!(
                out,
                " {} Clean Personas: ({clean_names})",
                "✔".bold().green()
            );
        }
    }

    let _ = writeln!(out, "{}", divider.dimmed());

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::synthesis::finding::{Finding, Severity};
    use std::path::PathBuf;

    #[test]
    fn test_render_terminal_dashboard_passed() {
        let synthesis = AuditSynthesis {
            findings: vec![],
            total_agents: 3,
            successful_agents: 3,
            passed_personas: vec!["agent-a".to_string(), "agent-b".to_string()],
            blocker_count: 0,
            major_count: 0,
            polish_count: 0,
        };

        let rendered = render_terminal_dashboard(&synthesis);
        assert!(rendered.contains("ARGUS AUDIT PASSED"));
        assert!(rendered.contains("Clean Personas:"));
        assert!(rendered.contains("agent-a, agent-b"));
    }

    #[test]
    fn test_render_terminal_dashboard_with_defects() {
        let finding = Finding {
            severity: Severity::P0Blocker,
            persona: "stock-ledger-auditor".to_string(),
            title: "Unpushed faktur deletion omits stock restoration".to_string(),
            file_path: PathBuf::from("simplidot_invoice_outbox_dao.dart"),
            line_number: Some(88),
            oracle_id: Some("state.reversibility".to_string()),
            description: "Stock not restored".to_string(),
            failure_scenario: "Delete invoice from queue".to_string(),
            proposed_fix: Some("await restoreStockMutation(id);".to_string()),
        };

        let synthesis = AuditSynthesis {
            findings: vec![finding],
            total_agents: 2,
            successful_agents: 2,
            passed_personas: vec!["sfd-detective".to_string()],
            blocker_count: 1,
            major_count: 0,
            polish_count: 0,
        };

        let rendered = render_terminal_dashboard(&synthesis);
        assert!(rendered.contains("ACTION REQUIRED"));
        assert!(rendered.contains("P0 - BLOCKER"));
        assert!(rendered.contains("simplidot_invoice_outbox_dao.dart:88"));
        assert!(rendered.contains("stock-ledger-auditor"));
    }
}
