#![allow(clippy::unwrap_used, clippy::expect_used)]

use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_argus_init_scaffolds_expected_structure() {
    let dir = tempdir().expect("should create temp dir");
    let target_path = dir.path();

    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.args(["init", "--target-dir", target_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Argus initialized successfully"));

    let argus_dir = target_path.join(".argus");
    assert!(argus_dir.is_dir());
    assert!(argus_dir.join("personas").is_dir());
    assert!(argus_dir.join("context").join("sfd").is_dir());
    assert!(argus_dir.join("reports").is_dir());

    let config_file = argus_dir.join("config.yaml");
    assert!(config_file.is_file());
    let config_content = fs::read_to_string(&config_file).expect("should read config");
    assert!(config_content.contains("project: \"my-project\""));
    assert!(config_content.contains("squads:"));

    // Verify config is valid YAML matching our schema
    let parsed: serde_yaml::Value =
        serde_yaml::from_str(&config_content).expect("config should be valid YAML");
    assert_eq!(parsed["version"], 1);

    let oracles_file = argus_dir.join("oracles.yaml");
    assert!(oracles_file.is_file());
    let oracles_content = fs::read_to_string(&oracles_file).expect("should read oracles");
    assert!(oracles_content.contains("state.reversibility"));
}

#[test]
fn test_argus_init_idempotency_without_force() {
    let dir = tempdir().expect("should create temp dir");
    let target_path = dir.path();

    // First run creates files
    let mut cmd1 = Command::cargo_bin("argus").expect("binary should exist");
    cmd1.args(["init", "--target-dir", target_path.to_str().unwrap()])
        .assert()
        .success();

    // Modify config.yaml manually
    let config_file = target_path.join(".argus").join("config.yaml");
    fs::write(&config_file, "custom: content").expect("should write custom config");

    // Second run without --force should skip
    let mut cmd2 = Command::cargo_bin("argus").expect("binary should exist");
    cmd2.args(["init", "--target-dir", target_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("already exists"))
        .stdout(predicate::str::contains(
            "skipping; use --force to overwrite",
        ));

    // Content should remain untouched
    let preserved = fs::read_to_string(&config_file).expect("should read config");
    assert_eq!(preserved, "custom: content");
}

#[test]
fn test_argus_init_overwrites_with_force() {
    let dir = tempdir().expect("should create temp dir");
    let target_path = dir.path();

    // First run
    let mut cmd1 = Command::cargo_bin("argus").expect("binary should exist");
    cmd1.args(["init", "--target-dir", target_path.to_str().unwrap()])
        .assert()
        .success();

    // Modify config.yaml manually
    let config_file = target_path.join(".argus").join("config.yaml");
    fs::write(&config_file, "custom: content").expect("should write custom config");

    // Second run WITH --force
    let mut cmd2 = Command::cargo_bin("argus").expect("binary should exist");
    cmd2.args([
        "init",
        "--force",
        "--target-dir",
        target_path.to_str().unwrap(),
    ])
    .assert()
    .success()
    .stdout(predicate::str::contains("Generated:"));

    // Content should be restored to starter config
    let overwritten = fs::read_to_string(&config_file).expect("should read config");
    assert!(overwritten.contains("project: \"my-project\""));
}

#[test]
fn test_argus_init_auto_bootstraps_custom_personas() {
    let dir = tempdir().expect("should create temp dir");
    let target_path = dir.path();

    // Create a mock pi script that generates 2 tailored personas in .argus/personas/
    let mock_pi_path = target_path.join("mock_pi.sh");
    let script = r#"#!/usr/bin/env bash
mkdir -p .argus/personas
cat << 'EOF' > .argus/personas/sqlite-ledger-auditor.md
---
name: sqlite-ledger-auditor
title: SQLite Offline Ledger Auditor
squad: data
model_tier: standard
tools: read, grep, find, ls, bash
---

Audit SQLite transaction atomicity.
EOF

cat << 'EOF' > .argus/personas/bloc-state-auditor.md
---
name: bloc-state-auditor
title: BLoC State Management Auditor
squad: state
model_tier: standard
tools: read, grep, find, ls, bash
---

Audit stream subscription leaks.
EOF

echo "Synthesized 2 tailored personas successfully."
exit 0
"#;
    fs::write(&mock_pi_path, script).expect("write mock script");

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&mock_pi_path, fs::Permissions::from_mode(0o755))
            .expect("set permissions");
    }

    let mut cmd = Command::cargo_bin("argus").expect("binary should exist");
    cmd.env_remove("ARGUS_ACTIVE_AUDIT")
        .env("ARGUS_PI_BIN", &mock_pi_path)
        .args([
            "init",
            "--auto",
            "--target-dir",
            target_path.to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Profiling codebase with Persona Architect",
        ))
        .stdout(predicate::str::contains(
            "Persona Architect synthesized 2 tailored persona(s):",
        ))
        .stdout(predicate::str::contains("sqlite-ledger-auditor"))
        .stdout(predicate::str::contains("bloc-state-auditor"))
        .stdout(predicate::str::contains("Registered new squad 'auto' in"));

    // Verify persona files exist on disk
    let personas_dir = target_path.join(".argus").join("personas");
    assert!(personas_dir.join("sqlite-ledger-auditor.md").is_file());
    assert!(personas_dir.join("bloc-state-auditor.md").is_file());

    // Verify config.yaml was updated with squad 'auto'
    let config_file = target_path.join(".argus").join("config.yaml");
    let config_content = fs::read_to_string(&config_file).expect("read config");
    assert!(config_content.contains("auto:"));
    assert!(config_content.contains("sqlite-ledger-auditor"));
    assert!(config_content.contains("bloc-state-auditor"));
}
