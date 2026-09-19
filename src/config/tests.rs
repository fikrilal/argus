#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::{Path, PathBuf};
use tempfile::tempdir;

use super::loader::{find_config_file, load_config};
use super::schema::ArgusConfig;

#[test]
fn test_default_config_values() {
    let config = ArgusConfig::default();
    assert_eq!(config.version, 1);
    assert_eq!(config.project, "default-project");
    assert_eq!(config.sfd.dir, PathBuf::from(".argus/context/sfd"));
    assert_eq!(config.sfd.active, None);
    assert_eq!(config.models.fast, "google/gemini-2.5-flash");
    assert_eq!(config.models.standard, "anthropic/claude-3-7-sonnet");
    assert_eq!(config.models.deep, "anthropic/claude-3-7-sonnet:high");
    assert!(config.squads.contains_key("forms"));
    assert!(config.squads.contains_key("state"));
    assert!(config.squads.contains_key("sync"));
    assert!(config.squads.contains_key("spec"));
    assert!(config.squads.contains_key("all"));

    let all_squad = config.squads.get("all").unwrap();
    assert_eq!(all_squad.len(), 11);
}

#[test]
fn test_parse_custom_yaml_string() {
    let yaml = r#"
version: 2
project: "kalbe-superapps"
sfd:
  dir: "docs/spec"
  active: "stockist-kulakan.md"
models:
  fast: "custom/fast-model"
squads:
  custom_squad:
    - persona-a
    - persona-b
"#;

    let config: ArgusConfig = serde_yaml::from_str(yaml).expect("should parse valid YAML");
    assert_eq!(config.version, 2);
    assert_eq!(config.project, "kalbe-superapps");
    assert_eq!(config.sfd.dir, PathBuf::from("docs/spec"));
    assert_eq!(
        config.sfd.active,
        Some(PathBuf::from("stockist-kulakan.md"))
    );
    assert_eq!(config.models.fast, "custom/fast-model");
    // Standard and deep should fall back to defaults
    assert_eq!(config.models.standard, "anthropic/claude-3-7-sonnet");
    assert_eq!(config.models.deep, "anthropic/claude-3-7-sonnet:high");

    let custom_squad = config.squads.get("custom_squad").unwrap();
    assert_eq!(custom_squad, &["persona-a", "persona-b"]);
}

#[test]
fn test_load_config_fallback_when_file_not_found() {
    let dir = tempdir().expect("tempdir should be created");
    let config = load_config(None, dir.path()).expect("should fall back to default");
    assert_eq!(config, ArgusConfig::default());
}

#[test]
fn test_load_config_from_discovered_argus_file() {
    let dir = tempdir().expect("tempdir should be created");
    let argus_dir = dir.path().join(".argus");
    std::fs::create_dir_all(&argus_dir).expect("should create .argus dir");

    let config_file = argus_dir.join("config.yaml");
    let yaml = r#"
project: "discovered-project"
"#;
    std::fs::write(&config_file, yaml).expect("should write config file");

    let found = find_config_file(dir.path());
    assert_eq!(found, Some(config_file));

    let loaded = load_config(None, dir.path()).expect("should load discovered config");
    assert_eq!(loaded.project, "discovered-project");
    assert_eq!(loaded.version, 1);
}

#[test]
fn test_load_config_from_explicit_path() {
    let dir = tempdir().expect("tempdir should be created");
    let custom_file = dir.path().join("my-custom-config.yaml");
    let yaml = r#"
project: "explicit-project"
"#;
    std::fs::write(&custom_file, yaml).expect("should write custom config");

    let loaded = load_config(Some(&custom_file), dir.path()).expect("should load explicit config");
    assert_eq!(loaded.project, "explicit-project");
}

#[test]
fn test_load_config_missing_explicit_path_returns_error() {
    let non_existent = Path::new("/path/that/does/not/exist/config.yaml");
    let result = load_config(Some(non_existent), Path::new("."));
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Failed to read custom config file"));
}

#[test]
fn test_load_config_invalid_yaml_returns_error() {
    let dir = tempdir().expect("tempdir should be created");
    let broken_file = dir.path().join("broken.yaml");
    std::fs::write(&broken_file, "version: [unclosed list").expect("should write file");

    let result = load_config(Some(&broken_file), dir.path());
    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Failed to parse YAML in config file"));
}
