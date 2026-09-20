use anyhow::{Context, Result};
use chrono::Utc;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

use super::finding::Severity;
use super::synthesizer::AuditSynthesis;

/// Generates a standalone Markdown report capturing the full audit results.
#[must_use]
pub fn generate_markdown_report(
    synthesis: &AuditSynthesis,
    branch: &str,
    squad: &str,
    sfd_title: Option<&str>,
    sfd_path: Option<&str>,
) -> String {
    let mut out = String::new();
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S UTC");

    let _ = writeln!(out, "# 👁️ Argus Pre-Flight QA Audit Report\n");

    let _ = writeln!(out, "## Audit Metadata\n");
    let _ = writeln!(out, "| Parameter | Value |");
    let _ = writeln!(out, "|---|---|");
    let _ = writeln!(out, "| **Timestamp** | `{now}` |");
    let _ = writeln!(out, "| **Git Branch** | `{branch}` |");
    let _ = writeln!(out, "| **Squad** | `{squad}` |");
    if let Some(path) = sfd_path {
        let title = sfd_title.unwrap_or("Active Spec");
        let _ = writeln!(out, "| **Active SFD** | {title} (`{path}`) |");
    } else {
        let _ = writeln!(out, "| **Active SFD** | *None* |");
    }
    let status_badge = if synthesis.is_passed() {
        "**PASSED** (Ready for QA)"
    } else {
        "**ACTION REQUIRED** (Defects Found)"
    };
    let _ = writeln!(out, "| **Status** | {status_badge} |");
    let _ = writeln!(
        out,
        "| **Findings** | {} ({} blockers, {} major, {} polish) |\n",
        synthesis.total_findings(),
        synthesis.blocker_count,
        synthesis.major_count,
        synthesis.polish_count
    );

    let _ = writeln!(out, "## Executive Summary\n");
    if synthesis.is_passed() {
        let _ = writeln!(
            out,
            "All **{}** active adversarial personas completed inspection with **zero critical defects**. The code on branch `{branch}` satisfies all tested invariants and is cleared for pull request creation and human QA handoff.\n",
            synthesis.total_agents
        );
    } else {
        let _ = writeln!(
            out,
            "The Argus swarm identified **{}** defect(s) (**{}** blockers, **{}** major) across branch `{branch}`. Remediate the findings below before opening a pull request.\n",
            synthesis.total_findings(),
            synthesis.blocker_count,
            synthesis.major_count
        );
    }

    if !synthesis.findings.is_empty() {
        let _ = writeln!(out, "## Defect Findings\n");

        for (idx, finding) in synthesis.findings.iter().enumerate() {
            let num = idx + 1;
            let sev_tag = match finding.severity {
                Severity::P0Blocker => "P0 - BLOCKER",
                Severity::P1Major => "P1 - MAJOR",
                Severity::P2Polish => "P2 - POLISH",
            };

            let _ = writeln!(out, "### {num}. `[{sev_tag}]` {}\n", finding.title);
            let _ = writeln!(out, "- **Target:** `{}`", finding.target_display());
            let _ = writeln!(out, "- **Reporting Persona:** `{}`", finding.persona);
            if !finding.description.is_empty() {
                let _ = writeln!(out, "- **Description:** {}", finding.description);
            }
            if !finding.failure_scenario.is_empty() {
                let _ = writeln!(out, "- **Failure Scenario:** {}", finding.failure_scenario);
            }
            if let Some(ref fix) = finding.proposed_fix {
                let _ = writeln!(out, "- **Recommended Fix:**\n```\n{fix}\n```");
            }
            let _ = writeln!(out);
        }
    }

    if !synthesis.passed_personas.is_empty() {
        let _ = writeln!(out, "## Clean Personas\n");
        let _ = writeln!(
            out,
            "The following **{}** personas verified their respective domains with zero defects found:\n",
            synthesis.passed_personas.len()
        );
        for persona in &synthesis.passed_personas {
            let _ = writeln!(out, "- `✔` `{persona}`");
        }
        let _ = writeln!(out);
    }

    out
}

/// Writes the Markdown report content to the designated reports directory.
///
/// Returns the path to the written report file.
pub fn write_markdown_report(report_content: &str, reports_dir: &Path) -> Result<PathBuf> {
    if !reports_dir.exists() {
        fs::create_dir_all(reports_dir).with_context(|| {
            format!(
                "Failed to create reports directory at '{}'",
                reports_dir.display()
            )
        })?;
    }

    let timestamp = Utc::now().format("%Y-%m-%d-%H%M%S");
    let filename = format!("{timestamp}-audit.md");
    let file_path = reports_dir.join(filename);

    fs::write(&file_path, report_content)
        .with_context(|| format!("Failed to write audit report to '{}'", file_path.display()))?;

    Ok(file_path)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use crate::synthesis::finding::{Finding, Severity};
    use tempfile::tempdir;

    #[test]
    fn test_generate_markdown_report_passed() {
        let synthesis = AuditSynthesis {
            findings: vec![],
            total_agents: 4,
            successful_agents: 4,
            passed_personas: vec!["sfd-clause-detective".to_string()],
            blocker_count: 0,
            major_count: 0,
            polish_count: 0,
        };

        let report = generate_markdown_report(
            &synthesis,
            "DEV-AFM-AUTH_MIGRATION",
            "all",
            Some("Stockist Kulakan"),
            Some(".argus/context/sfd/kulakan.md"),
        );

        assert!(report.contains("# 👁️ Argus Pre-Flight QA Audit Report"));
        assert!(report.contains("`DEV-AFM-AUTH_MIGRATION`"));
        assert!(report.contains("Stockist Kulakan (`.argus/context/sfd/kulakan.md`)"));
        assert!(report.contains("**PASSED** (Ready for QA)"));
        assert!(report.contains("- `✔` `sfd-clause-detective`"));
    }

    #[test]
    fn test_generate_markdown_report_with_defects() {
        let finding = Finding {
            severity: Severity::P0Blocker,
            persona: "stock-ledger-auditor".to_string(),
            title: "Inventory not rolled back".to_string(),
            file_path: PathBuf::from("invoice_dao.dart"),
            line_number: Some(88),
            oracle_id: Some("state.reversibility".to_string()),
            description: "Missing stock return mutation".to_string(),
            failure_scenario: "Delete invoice before sync".to_string(),
            proposed_fix: Some("await restoreStock();".to_string()),
        };

        let synthesis = AuditSynthesis {
            findings: vec![finding],
            total_agents: 2,
            successful_agents: 2,
            passed_personas: vec![],
            blocker_count: 1,
            major_count: 0,
            polish_count: 0,
        };

        let report = generate_markdown_report(&synthesis, "main", "state", None, None);

        assert!(report.contains("**ACTION REQUIRED** (Defects Found)"));
        assert!(report.contains("`[P0 - BLOCKER]` Inventory not rolled back"));
        assert!(report.contains("- **Target:** `invoice_dao.dart:88`"));
        assert!(report.contains("await restoreStock();"));
    }

    #[test]
    fn test_write_markdown_report_creates_file() {
        let dir = tempdir().expect("tempdir");
        let reports_dir = dir.path().join("reports");

        let content = "# Test Report\n\nContent here.";
        let written_path =
            write_markdown_report(content, &reports_dir).expect("should write report");

        assert!(written_path.exists());
        assert!(written_path.is_file());
        assert!(
            written_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .ends_with("-audit.md")
        );

        let read_back = fs::read_to_string(&written_path).expect("should read back");
        assert_eq!(read_back, content);
    }
}
