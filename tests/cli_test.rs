#![allow(clippy::unwrap_used, clippy::expect_used)]

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_version() {
    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("argus 0.1.0"));
}

#[test]
fn test_cli_help() {
    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Argus unleashes a multi-agent adversarial swarm",
        ))
        .stdout(predicate::str::contains("audit"))
        .stdout(predicate::str::contains("init"))
        .stdout(predicate::str::contains("personas"))
        .stdout(predicate::str::contains("resume"));
}

#[test]
fn test_cli_audit_help() {
    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.args(["audit", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Run an adversarial QA audit"))
        .stdout(predicate::str::contains("--squad"))
        .stdout(predicate::str::contains("--path"))
        .stdout(predicate::str::contains("--sfd"))
        .stdout(predicate::str::contains("--concurrency"));
}

#[test]
fn test_cli_audit_conflicts_full_and_staged() {
    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.args(["audit", "--full", "--staged"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn test_cli_audit_conflicts_staged_and_base() {
    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.args(["audit", "--staged", "--base", "main"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn test_cli_audit_conflicts_full_and_base() {
    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.args(["audit", "--full", "--base", "main"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("cannot be used with"));
}

#[test]
fn test_cli_init_help() {
    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.args(["init", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialize Argus configuration"))
        .stdout(predicate::str::contains("--force"))
        .stdout(predicate::str::contains("--target-dir"));
}

#[test]
fn test_cli_personas_help() {
    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.args(["personas", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Inspect and list available agent personas",
        ))
        .stdout(predicate::str::contains("list"))
        .stdout(predicate::str::contains("show"));
}

#[test]
fn test_cli_personas_list_execution() {
    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.args(["personas", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Available personas: (total:"))
        .stdout(predicate::str::contains("stock-ledger-auditor"))
        .stdout(predicate::str::contains("form-boundary-saboteur"))
        .stdout(predicate::str::contains("sfd-clause-detective"));
}

#[test]
fn test_cli_personas_list_with_squad_filter() {
    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.args(["personas", "list", "--squad", "state"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Available personas: (total: 2)"))
        .stdout(predicate::str::contains("stock-ledger-auditor"))
        .stdout(predicate::str::contains("orphan-cascade-hunter"));
}

#[test]
fn test_cli_personas_show_execution() {
    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.args(["personas", "show", "stock-ledger-auditor"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Persona: stock-ledger-auditor"))
        .stdout(predicate::str::contains("Reversibility Law"))
        .stdout(predicate::str::contains("Tier:       Deep"));
}

#[test]
fn test_cli_personas_show_unknown_returns_error() {
    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.args(["personas", "show", "unknown-persona-xyz"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Persona 'unknown-persona-xyz' not found in registry",
        ));
}

#[test]
fn test_cli_resume_help() {
    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.args(["resume", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Resume an active agent session"))
        .stdout(predicate::str::contains("--persona"))
        .stdout(predicate::str::contains("--branch"));
}

#[test]
fn test_cli_resume_unknown_persona_returns_error() {
    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.args(["resume", "--persona", "unknown-persona-xyz"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Persona 'unknown-persona-xyz' not found in registry",
        ));
}

#[test]
fn test_cli_no_args_shows_usage_error() {
    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage: argus"));
}

#[test]
fn test_cli_audit_blocks_recursive_invocation() {
    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.env("ARGUS_ACTIVE_AUDIT", "1")
        .args(["audit", "--full"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Recursive audit blocked"));
}
