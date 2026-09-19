use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use super::schema::ArgusConfig;

const CONFIG_CANDIDATES: &[&str] = &[
    ".argus/config.yaml",
    ".argus/config.yml",
    ".swarm/config.yaml",
    ".swarm/config.yml",
];

/// Discovers and loads the Argus configuration.
///
/// If `custom_path` is specified:
/// - Loads strictly from that path, returning an error if it does not exist or fails to parse.
///
/// If `custom_path` is `None`:
/// - Searches candidate paths in `start_dir`.
/// - If a configuration file is found, it is parsed and returned.
/// - If no configuration file is found, returns [`ArgusConfig::default()`].
pub fn load_config(custom_path: Option<&Path>, start_dir: &Path) -> Result<ArgusConfig> {
    if let Some(path) = custom_path {
        let content = std::fs::read_to_string(path).with_context(|| {
            format!("Failed to read custom config file at '{}'", path.display())
        })?;

        let config: ArgusConfig = serde_yaml::from_str(&content).with_context(|| {
            format!(
                "Failed to parse YAML in config file at '{}'",
                path.display()
            )
        })?;

        return Ok(config);
    }

    if let Some(found_path) = find_config_file(start_dir) {
        let content = std::fs::read_to_string(&found_path).with_context(|| {
            format!(
                "Failed to read discovered config file at '{}'",
                found_path.display()
            )
        })?;

        let config: ArgusConfig = serde_yaml::from_str(&content).with_context(|| {
            format!(
                "Failed to parse YAML in config file at '{}'",
                found_path.display()
            )
        })?;

        return Ok(config);
    }

    Ok(ArgusConfig::default())
}

/// Searches `start_dir` for a known configuration file candidate.
pub fn find_config_file(start_dir: &Path) -> Option<PathBuf> {
    for candidate in CONFIG_CANDIDATES {
        let path = start_dir.join(candidate);
        if path.is_file() {
            return Some(path);
        }
    }
    None
}
