#![allow(clippy::unwrap_used, clippy::expect_used)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

fn setup_mock_pi(dir: &Path, mode: &str) -> PathBuf {
    let script_path = dir.join("mock_pi.sh");
    let script = match mode {
        "pass" => {
            r#"#!/usr/bin/env bash
echo "[STATUS: PASS] All tested criteria satisfied with zero violations."
exit 0
"#
        }
        "violation" => {
            r#"#!/usr/bin/env bash
echo "- **Status:** [VIOLATION]"
echo "- **Target:** module/simplidot/lib/pages/customer/add_customer_page.dart:242"
echo "- **Issue:** RT and RW fields allow text input and lack length constraints."
echo "- **Failure Scenario:** User types string into numeric field, causing backend sync rejection."
echo "- **Recommended Fix:** Add LengthLimitingTextInputFormatter(3)."
exit 0
"#
        }
        _ => {
            r#"#!/usr/bin/env bash
echo "Unknown mode"
exit 1
"#
        }
    };

    fs::write(&script_path, script).expect("should write mock script");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755))
            .expect("should set executable perms");
    }

    script_path
}

#[test]
fn test_audit_e2e_clean_pass() {
    let dir = tempdir().expect("should create temp dir");
    let mock_pi = setup_mock_pi(dir.path(), "pass");

    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.current_dir(dir.path())
        .env("ARGUS_PI_BIN", &mock_pi)
        .args(["audit", "--squad", "forms", "--concurrency", "2"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Running Argus audit for:"))
        .stdout(predicate::str::contains("squad='forms'"))
        .stdout(predicate::str::contains("Deploying agent squad:"))
        .stdout(predicate::str::contains("ARGUS AUDIT PASSED"))
        .stdout(predicate::str::contains(
            "Saved persistent audit report to:",
        ));

    // Verify report was created in .argus/reports/
    let reports_dir = dir.path().join(".argus").join("reports");
    assert!(reports_dir.is_dir());

    let entries: Vec<_> = fs::read_dir(&reports_dir)
        .expect("read reports dir")
        .filter_map(Result::ok)
        .collect();

    assert_eq!(entries.len(), 1);
    let report_content = fs::read_to_string(entries[0].path()).expect("read report");
    assert!(report_content.contains("# 👁️ Argus Pre-Flight QA Audit Report"));
    assert!(report_content.contains("**PASSED** (Ready for QA)"));
}

#[test]
fn test_audit_e2e_defects_found_action_required() {
    let dir = tempdir().expect("should create temp dir");
    let mock_pi = setup_mock_pi(dir.path(), "violation");

    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.current_dir(dir.path())
        .env("ARGUS_PI_BIN", &mock_pi)
        .args(["audit", "--squad", "forms", "--concurrency", "2"])
        .assert()
        .failure()
        .stdout(predicate::str::contains("ARGUS AUDIT ACTION REQUIRED"))
        .stdout(predicate::str::contains("[P1 - MAJOR]"))
        .stdout(predicate::str::contains("add_customer_page.dart:242"))
        .stdout(predicate::str::contains(
            "LengthLimitingTextInputFormatter(3)",
        ))
        .stdout(predicate::str::contains(
            "Saved persistent audit report to:",
        ))
        .stderr(predicate::str::contains(
            "Action required before QA handoff",
        ));

    // Verify report was created on disk
    let reports_dir = dir.path().join(".argus").join("reports");
    assert!(reports_dir.is_dir());

    let entries: Vec<_> = fs::read_dir(&reports_dir)
        .expect("read reports dir")
        .filter_map(Result::ok)
        .collect();

    assert_eq!(entries.len(), 1);
    let report_content = fs::read_to_string(entries[0].path()).expect("read report");
    assert!(report_content.contains("**ACTION REQUIRED** (Defects Found)"));
    assert!(report_content.contains("`[P1 - MAJOR]`"));
    assert!(report_content.contains("add_customer_page.dart:242"));
}

#[test]
fn test_audit_e2e_with_sfd_flag() {
    let dir = tempdir().expect("should create temp dir");
    let mock_pi = setup_mock_pi(dir.path(), "pass");

    let sfd_path = dir.path().join("spec.md");
    fs::write(
        &sfd_path,
        "# Stockist Kulakan Feature Spec\n\nBusiness rules...",
    )
    .expect("write spec");

    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.current_dir(dir.path())
        .env("ARGUS_PI_BIN", &mock_pi)
        .args([
            "audit",
            "--squad",
            "forms",
            "--sfd",
            sfd_path.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Active SFD: 'Stockist Kulakan Feature Spec'",
        ))
        .stdout(predicate::str::contains("ARGUS AUDIT PASSED"));

    // Verify report mentions the SFD
    let reports_dir = dir.path().join(".argus").join("reports");
    let entries: Vec<_> = fs::read_dir(&reports_dir)
        .expect("read reports dir")
        .filter_map(Result::ok)
        .collect();

    let report_content = fs::read_to_string(entries[0].path()).expect("read report");
    assert!(report_content.contains("Stockist Kulakan Feature Spec"));
}
