use std::collections::HashMap;
use std::path::PathBuf;

use super::finding::{Finding, Severity, parse_agent_findings};

/// An input report from an executed agent to be synthesized.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentReport {
    /// Persona identifier
    pub persona_name: String,
    /// Output text produced by the agent
    pub raw_output: String,
    /// Whether the agent process succeeded
    pub is_success: bool,
}

/// The synthesized result of a multi-agent swarm audit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditSynthesis {
    /// Deduplicated and ranked findings (sorted P0 -> P1 -> P2)
    pub findings: Vec<Finding>,
    /// Total number of agents executed
    pub total_agents: usize,
    /// Number of agents that executed successfully
    pub successful_agents: usize,
    /// List of persona names that found zero violations
    pub passed_personas: Vec<String>,
    /// Count of P0 Blocker findings
    pub blocker_count: usize,
    /// Count of P1 Major findings
    pub major_count: usize,
    /// Count of P2 Polish findings
    pub polish_count: usize,
}

impl AuditSynthesis {
    /// Returns true if there are zero P0 Blockers and zero P1 Major findings.
    #[must_use]
    #[allow(dead_code)]
    pub fn is_passed(&self) -> bool {
        self.blocker_count == 0 && self.major_count == 0
    }

    /// Total count of all findings.
    #[must_use]
    #[allow(dead_code)]
    pub fn total_findings(&self) -> usize {
        self.findings.len()
    }
}

/// Key used to deduplicate findings targeting the same location or issue.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DeduplicationKey {
    file_path: PathBuf,
    line_number: Option<usize>,
    title_slug: String,
}

/// Aggregates, deduplicates, and ranks raw agent reports into an [`AuditSynthesis`].
#[must_use]
pub fn synthesize_reports(reports: &[AgentReport]) -> AuditSynthesis {
    let total_agents = reports.len();
    let mut successful_agents = 0;
    let mut passed_personas = Vec::new();

    let mut deduplication_map: HashMap<DeduplicationKey, Finding> = HashMap::new();

    for report in reports {
        if !report.is_success {
            continue;
        }
        successful_agents += 1;

        let parsed = parse_agent_findings(&report.persona_name, &report.raw_output);

        if parsed.is_empty() {
            passed_personas.push(report.persona_name.clone());
            continue;
        }

        for finding in parsed {
            let key = DeduplicationKey {
                file_path: finding.file_path.clone(),
                line_number: finding.line_number,
                title_slug: if finding.line_number.is_none() {
                    finding.title.to_lowercase()
                } else {
                    String::new()
                },
            };

            if let Some(existing) = deduplication_map.get_mut(&key) {
                // Elevate severity to the highest (P0 < P1 < P2)
                if finding.severity < existing.severity {
                    existing.severity = finding.severity;
                }

                // Merge persona names if not already included
                if !existing.persona.contains(&finding.persona) {
                    existing.persona.push_str(", ");
                    existing.persona.push_str(&finding.persona);
                }

                // Prefer the proposed fix if existing is empty
                if existing.proposed_fix.is_none() && finding.proposed_fix.is_some() {
                    existing.proposed_fix = finding.proposed_fix;
                }
            } else {
                deduplication_map.insert(key, finding);
            }
        }
    }

    let mut findings: Vec<Finding> = deduplication_map.into_values().collect();

    // Sort findings strictly: P0 -> P1 -> P2, then by file path and line number
    findings.sort_by(|a, b| {
        a.severity
            .cmp(&b.severity)
            .then_with(|| a.file_path.cmp(&b.file_path))
            .then_with(|| a.line_number.cmp(&b.line_number))
    });

    let blocker_count = findings
        .iter()
        .filter(|f| f.severity == Severity::P0Blocker)
        .count();
    let major_count = findings
        .iter()
        .filter(|f| f.severity == Severity::P1Major)
        .count();
    let polish_count = findings
        .iter()
        .filter(|f| f.severity == Severity::P2Polish)
        .count();

    AuditSynthesis {
        findings,
        total_agents,
        successful_agents,
        passed_personas,
        blocker_count,
        major_count,
        polish_count,
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn test_synthesize_reports_deduplication_and_ranking() {
        // Agent 1: id-regulatory-sentinel reports P1 on add_customer_page.dart:242
        let report_1 = AgentReport {
            persona_name: "id-regulatory-sentinel".to_string(),
            raw_output: r"
- **Status:** [VIOLATION]
- **Target:** `add_customer_page.dart:242`
- **Issue:** RT/RW lacks length limit.
- **Failure Scenario:** Pasting long string.
- **Recommended Fix:** LengthLimitingTextInputFormatter(3)
"
            .to_string(),
            is_success: true,
        };

        // Agent 2: form-boundary-saboteur reports P0 on same target add_customer_page.dart:242
        let report_2 = AgentReport {
            persona_name: "form-boundary-saboteur".to_string(),
            raw_output: r"
- **Status:** [VIOLATION - BLOCKER]
- **Target:** `add_customer_page.dart:242`
- **Issue:** RT/RW boundary allows arbitrary characters causing database crash.
- **Failure Scenario:** Crash on sync.
- **Recommended Fix:** FilteringTextInputFormatter.digitsOnly
"
            .to_string(),
            is_success: true,
        };

        // Agent 3: clean agent
        let report_3 = AgentReport {
            persona_name: "sfd-clause-detective".to_string(),
            raw_output: "[STATUS: PASS] All SFD clauses implemented.".to_string(),
            is_success: true,
        };

        let synthesis = synthesize_reports(&[report_1, report_2, report_3]);

        assert_eq!(synthesis.total_agents, 3);
        assert_eq!(synthesis.successful_agents, 3);
        assert_eq!(synthesis.passed_personas, &["sfd-clause-detective"]);

        // Findings must be deduplicated from 2 -> 1
        assert_eq!(synthesis.findings.len(), 1);

        let merged = &synthesis.findings[0];
        assert_eq!(merged.file_path, PathBuf::from("add_customer_page.dart"));
        assert_eq!(merged.line_number, Some(242));
        // Severity must be elevated to P0Blocker
        assert_eq!(merged.severity, Severity::P0Blocker);
        // Personas merged
        assert!(merged.persona.contains("id-regulatory-sentinel"));
        assert!(merged.persona.contains("form-boundary-saboteur"));

        assert_eq!(synthesis.blocker_count, 1);
        assert_eq!(synthesis.major_count, 0);
        assert_eq!(synthesis.polish_count, 0);
        assert!(!synthesis.is_passed());
    }

    #[test]
    fn test_synthesize_reports_clean_run() {
        let report_1 = AgentReport {
            persona_name: "agent-1".to_string(),
            raw_output: "[STATUS: PASS]".to_string(),
            is_success: true,
        };
        let report_2 = AgentReport {
            persona_name: "agent-2".to_string(),
            raw_output: "[STATUS: PASS]".to_string(),
            is_success: true,
        };

        let synthesis = synthesize_reports(&[report_1, report_2]);
        assert_eq!(synthesis.findings.len(), 0);
        assert_eq!(synthesis.passed_personas.len(), 2);
        assert!(synthesis.is_passed());
    }
}
