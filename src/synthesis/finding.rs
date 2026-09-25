use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The severity level of a defect discovered during an audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[allow(dead_code)]
pub enum Severity {
    /// P0: Blocker — Data loss, inventory ledger imbalance, broken reversibility,
    /// sync queue deadlock, unhandled crash on HTTP 200 OK.
    P0Blocker,

    /// P1: Major — Bypassed form validation sending corrupt data to backend,
    /// cascading dropdown state corruption, missing SFD business rules.
    P1Major,

    /// P2: Polish — Missing soft limits, cosmetic formatting inconsistencies,
    /// non-fatal code organization suggestions.
    P2Polish,
}

impl Severity {
    /// Returns the short code (e.g. "P0", "P1", "P2").
    #[must_use]
    #[allow(dead_code)]
    pub fn code(self) -> &'static str {
        match self {
            Self::P0Blocker => "P0",
            Self::P1Major => "P1",
            Self::P2Polish => "P2",
        }
    }

    /// Returns the severity name (e.g. "BLOCKER", "MAJOR", "POLISH").
    #[must_use]
    #[allow(dead_code)]
    pub fn label(self) -> &'static str {
        match self {
            Self::P0Blocker => "BLOCKER",
            Self::P1Major => "MAJOR",
            Self::P2Polish => "POLISH",
        }
    }

    /// Returns a colorized terminal badge for this severity level.
    #[must_use]
    #[allow(dead_code)]
    pub fn badge(self) -> String {
        match self {
            Self::P0Blocker => format!("[{} - {}]", self.code(), self.label())
                .bold()
                .red()
                .to_string(),
            Self::P1Major => format!("[{} - {}]", self.code(), self.label())
                .bold()
                .yellow()
                .to_string(),
            Self::P2Polish => format!("[{} - {}]", self.code(), self.label())
                .bold()
                .cyan()
                .to_string(),
        }
    }
}

/// A structured defect finding discovered by an agent persona.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct Finding {
    /// Severity classification
    pub severity: Severity,
    /// Reporting persona name
    pub persona: String,
    /// Concise summary title
    pub title: String,
    /// Target file path where the issue occurs
    pub file_path: PathBuf,
    /// Optional line number
    pub line_number: Option<usize>,
    /// Optional matching Behavioral Oracle rule ID
    pub oracle_id: Option<String>,
    /// Full description of the defect
    pub description: String,
    /// Reproduction or trigger scenario
    pub failure_scenario: String,
    /// Concrete recommended code fix snippet
    pub proposed_fix: Option<String>,
}

impl Finding {
    /// Returns formatted file path and line number (e.g. `path/to/file.dart:42`).
    #[must_use]
    #[allow(dead_code)]
    pub fn target_display(&self) -> String {
        if let Some(line) = self.line_number {
            format!("{}:{line}", self.file_path.display())
        } else {
            self.file_path.display().to_string()
        }
    }
}

/// Parses an agent's Markdown output into structured [`Finding`] records.
///
/// Looks for blocks containing:
/// - `Status: VIOLATION`
/// - `Target: <path>:<line>`
/// - `Issue: <summary>`
/// - `Failure Scenario: <scenario>`
/// - `Recommended Fix: <fix>`
#[must_use]
#[allow(dead_code)]
pub fn parse_agent_findings(persona_name: &str, raw_output: &str) -> Vec<Finding> {
    let has_pass = raw_output.contains("[STATUS: PASS]")
        || raw_output.contains("[STATUS:PASS]")
        || raw_output.contains("Status: PASS");

    if has_pass && !raw_output.contains("VIOLATION") {
        return Vec::new();
    }

    let mut findings = Vec::new();
    let sections = split_into_violation_sections(raw_output);

    for section in sections {
        if let Some(finding) = parse_single_violation_block(persona_name, &section) {
            findings.push(finding);
        }
    }

    findings
}

fn split_into_violation_sections(text: &str) -> Vec<String> {
    let mut sections = Vec::new();
    let mut current = Vec::new();

    for line in text.lines() {
        let is_violation = (line.contains("**Status:**") || line.contains("Status:"))
            && line.contains("VIOLATION");

        if is_violation && !current.is_empty() {
            sections.push(current.join("\n"));
            current.clear();
        }
        current.push(line);
    }

    if !current.is_empty() {
        sections.push(current.join("\n"));
    }

    sections
}

fn parse_single_violation_block(persona_name: &str, text: &str) -> Option<Finding> {
    if !text.contains("VIOLATION") {
        return None;
    }

    let mut target = None;
    let mut issue = None;
    let mut scenario = None;
    let mut fix = None;

    let mut in_fix_section = false;
    let mut fix_lines = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();

        if in_fix_section {
            fix_lines.push(line);
            continue;
        }

        if let Some(val) = extract_field_value(trimmed, "Target:") {
            target = Some(val.trim_matches('`').trim().to_string());
        } else if let Some(val) = extract_field_value(trimmed, "Issue:") {
            issue = Some(val);
        } else if let Some(val) = extract_field_value(trimmed, "Failure Scenario:") {
            scenario = Some(val);
        } else if let Some(val) = extract_field_value(trimmed, "Recommended Fix:") {
            in_fix_section = true;
            if !val.is_empty() {
                fix_lines.push(line);
            }
        }
    }

    if !fix_lines.is_empty() {
        fix = Some(fix_lines.join("\n").trim().to_string());
    }

    let title = issue
        .clone()
        .unwrap_or_else(|| format!("Defect identified by {persona_name}"));

    let (file_path, line_number) = if let Some(ref t) = target {
        parse_target_path(t)
    } else {
        (PathBuf::from("unknown"), None)
    };

    let severity = classify_severity(persona_name, &title, text);

    Some(Finding {
        severity,
        persona: persona_name.to_string(),
        title,
        file_path,
        line_number,
        oracle_id: None,
        description: issue.unwrap_or_default(),
        failure_scenario: scenario.unwrap_or_default(),
        proposed_fix: fix,
    })
}

fn extract_field_value(line: &str, field_key: &str) -> Option<String> {
    let lower_key = field_key.to_lowercase();
    for (byte_idx, _) in line.char_indices() {
        let candidate = &line[byte_idx..];
        // Fast path for ASCII field keys: avoids heap allocation and guarantees character boundary safety
        if candidate.len() >= field_key.len()
            && candidate.is_char_boundary(field_key.len())
            && candidate[..field_key.len()].eq_ignore_ascii_case(field_key)
        {
            let after = &candidate[field_key.len()..];
            let cleaned = after
                .trim_start_matches('*')
                .trim_start_matches(':')
                .trim_start_matches('*')
                .trim();
            return Some(cleaned.to_string());
        }

        // Safe Unicode fallback path for non-ASCII field keys
        if candidate.to_lowercase().starts_with(&lower_key) {
            let mut char_indices = candidate.char_indices();
            let mut key_chars = lower_key.chars();
            while key_chars.next().is_some() {
                char_indices.next();
            }
            let end_offset = char_indices.next().map_or(candidate.len(), |(idx, _)| idx);
            let after = &candidate[end_offset..];
            let cleaned = after
                .trim_start_matches('*')
                .trim_start_matches(':')
                .trim_start_matches('*')
                .trim();
            return Some(cleaned.to_string());
        }
    }

    None
}

fn parse_target_path(target: &str) -> (PathBuf, Option<usize>) {
    let cleaned = target.trim_matches('`').trim();
    if let Some(res) = cleaned.rsplit_once(':').and_then(|(p, l)| {
        l.parse::<usize>()
            .ok()
            .map(|num| (PathBuf::from(p.trim()), Some(num)))
    }) {
        return res;
    }
    (PathBuf::from(cleaned), None)
}

fn classify_severity(persona_name: &str, title: &str, text: &str) -> Severity {
    let lower_text = text.to_lowercase();
    let lower_title = title.to_lowercase();

    // 1. Explicit status tags in agent report
    if lower_text.contains("p0") || lower_text.contains("blocker") {
        return Severity::P0Blocker;
    }
    if lower_text.contains("p2") || lower_text.contains("polish") {
        return Severity::P2Polish;
    }

    // 2. Persona and domain rules
    if persona_name.contains("stock-ledger")
        || persona_name.contains("sync-deadlock")
        || lower_title.contains("data loss")
        || lower_title.contains("reversibility")
        || lower_title.contains("deadlock")
        || lower_title.contains("unhandled exception")
    {
        return Severity::P0Blocker;
    }

    if lower_title.contains("suggestion")
        || lower_title.contains("cosmetic")
        || lower_title.contains("naming")
    {
        return Severity::P2Polish;
    }

    // Default to P1 Major
    Severity::P1Major
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn test_severity_ordering_and_badges() {
        assert!(Severity::P0Blocker < Severity::P1Major);
        assert!(Severity::P1Major < Severity::P2Polish);

        assert_eq!(Severity::P0Blocker.code(), "P0");
        assert_eq!(Severity::P0Blocker.label(), "BLOCKER");
        assert!(Severity::P0Blocker.badge().contains("P0 - BLOCKER"));

        assert_eq!(Severity::P1Major.code(), "P1");
        assert_eq!(Severity::P1Major.label(), "MAJOR");
        assert!(Severity::P1Major.badge().contains("P1 - MAJOR"));
    }

    #[test]
    fn test_parse_agent_findings_single_violation() {
        let output = r#"
- **Status:** [VIOLATION]
- **Target:** `module/simplidot/lib/pages/customer/add_customer_page.dart:242`
- **Issue:** RT and RW fields allow text input and have no length constraint.
- **Failure Scenario:** User types "RT 05" or 20 characters into RT, crashing backend sync validation.
- **Recommended Fix:**
```dart
inputFormatters: [
  FilteringTextInputFormatter.digitsOnly,
  LengthLimitingTextInputFormatter(3),
]
```
"#;

        let findings = parse_agent_findings("id-regulatory-sentinel", output);
        assert_eq!(findings.len(), 1);

        let f = &findings[0];
        assert_eq!(f.persona, "id-regulatory-sentinel");
        assert_eq!(f.severity, Severity::P1Major);
        assert_eq!(
            f.file_path,
            PathBuf::from("module/simplidot/lib/pages/customer/add_customer_page.dart")
        );
        assert_eq!(f.line_number, Some(242));
        assert_eq!(
            f.target_display(),
            "module/simplidot/lib/pages/customer/add_customer_page.dart:242"
        );
        assert!(f.title.contains("RT and RW fields"));
        assert!(f.failure_scenario.contains("User types"));
        assert!(
            f.proposed_fix
                .as_ref()
                .unwrap()
                .contains("FilteringTextInputFormatter.digitsOnly")
        );
    }

    #[test]
    fn test_parse_agent_findings_p0_blocker_classification() {
        let output = r"
- **Status:** [VIOLATION]
- **Target:** `module/simplidot/lib/local_db/daos/invoice/simplidot_invoice_outbox_dao.dart:88`
- **Issue:** Deleting an unpushed local invoice breaks the Reversibility Law by omitting stock restoration.
- **Failure Scenario:** Deleting an invoice from queue leaves inventory permanently deducted.
- **Recommended Fix:**
```dart
await restoreStockMutation(invoiceId);
```
";

        let findings = parse_agent_findings("stock-ledger-auditor", output);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].severity, Severity::P0Blocker);
    }

    #[test]
    fn test_parse_agent_findings_pass_status() {
        let output = "[STATUS: PASS] All form inputs adhere strictly to boundary formatters.";
        let findings = parse_agent_findings("form-boundary-saboteur", output);
        assert!(findings.is_empty());
    }

    #[test]
    fn test_parse_agent_findings_multiple_violations() {
        let output = r"
- **Status:** [VIOLATION]
- **Target:** `lib/a.dart:10`
- **Issue:** Issue A
- **Failure Scenario:** Scenario A
- **Recommended Fix:** Fix A

- **Status:** [VIOLATION]
- **Target:** `lib/b.dart:20`
- **Issue:** Issue B
- **Failure Scenario:** Scenario B
- **Recommended Fix:** Fix B
";

        let findings = parse_agent_findings("test-agent", output);
        assert_eq!(findings.len(), 2);
        assert_eq!(findings[0].file_path, PathBuf::from("lib/a.dart"));
        assert_eq!(findings[0].line_number, Some(10));
        assert_eq!(findings[1].file_path, PathBuf::from("lib/b.dart"));
        assert_eq!(findings[1].line_number, Some(20));
    }

    #[test]
    fn test_extract_field_value_unicode_asymmetric_case_folding() {
        // German capital sharp S 'ẞ' (3 bytes in UTF-8) case-folds to lowercase 'ß' (2 bytes in UTF-8)
        let line_with_eszett = "- **GROßER FEHLER** - **Target:** `src/models/user.rs:42`";
        let target = extract_field_value(line_with_eszett, "Target:");
        assert_eq!(target, Some("`src/models/user.rs:42`".to_string()));

        // Turkish capital I with dot 'İ' (2 bytes) case-folds to 'i\u{307}' (3 bytes)
        let line_with_turkish = "İSTANBUL PROJESİ - **Issue:** Critical validation bypass";
        let issue = extract_field_value(line_with_turkish, "Issue:");
        assert_eq!(issue, Some("Critical validation bypass".to_string()));

        // Multi-byte Unicode emojis
        let line_with_emojis =
            "🔍 🎯 ⚠️ - **Failure Scenario:** Buffer overflow occurs on negative inputs";
        let scenario = extract_field_value(line_with_emojis, "Failure Scenario:");
        assert_eq!(
            scenario,
            Some("Buffer overflow occurs on negative inputs".to_string())
        );
    }
}
