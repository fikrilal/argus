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
