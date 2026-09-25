use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

use super::schema::ArgusConfig;

const CONFIG_CANDIDATES: &[&str] = &[
    ".argus/config.yaml",
    ".argus/config.yml",
    ".swarm/config.yaml",
    ".swarm/config.yml",
];

const DIRECTORY_CONFIG_CANDIDATES: &[&str] = &[
    "config.yaml",
    "config.yml",
    ".argus/config.yaml",
    ".argus/config.yml",
    ".swarm/config.yaml",
    ".swarm/config.yml",
];

/// Discovers and loads the Argus configuration.
///
/// If `custom_path` is specified:
/// - Resolves relative paths against `start_dir`.
/// - If `path` is a directory, resolves candidate config within that directory (`config.yaml`, `.argus/config.yaml`, etc.).
/// - Loads strictly from that path, returning an error if it does not exist or fails to parse.
///
/// If `custom_path` is `None`:
/// - Searches candidate paths in `start_dir` and its ancestors.
/// - If a configuration file is found, it is parsed and returned.
/// - If no configuration file is found, returns [`ArgusConfig::default()`].
pub fn load_config(custom_path: Option<&Path>, start_dir: &Path) -> Result<ArgusConfig> {
    if let Some(path) = custom_path {
        let resolved_path = if path.is_relative() {
            start_dir.join(path)
        } else {
            path.to_path_buf()
        };

        let file_path = if resolved_path.is_dir() {
            find_config_in_custom_dir(&resolved_path).ok_or_else(|| {
                anyhow::anyhow!(
                    "Custom config path '{}' is a directory, but no candidate config file (config.yaml, .argus/config.yaml, etc.) was found within it",
                    path.display()
                )
            })?
        } else if !resolved_path.is_file() {
            anyhow::bail!(
                "Custom config file does not exist or is not a regular file at '{}'",
                resolved_path.display()
            );
        } else {
            resolved_path
        };

        let content = std::fs::read_to_string(&file_path).with_context(|| {
            format!(
                "Failed to read custom config file at '{}'",
                file_path.display()
            )
        })?;

        let config: ArgusConfig = serde_yaml::from_str(&content).with_context(|| {
            format!(
                "Failed to parse YAML in config file at '{}'",
                file_path.display()
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

/// Searches `start_dir` and its parent directories for a known configuration file candidate.
pub fn find_config_file(start_dir: &Path) -> Option<PathBuf> {
    for dir in start_dir.ancestors() {
        if let Some(path) = find_config_in_dir(dir) {
            return Some(path);
        }
    }
    None
}

fn find_config_in_dir(dir: &Path) -> Option<PathBuf> {
    for candidate in CONFIG_CANDIDATES {
        let path = dir.join(candidate);
        if path.is_file() {
            return Some(path);
        }
    }
    None
}

fn find_config_in_custom_dir(dir: &Path) -> Option<PathBuf> {
    for candidate in DIRECTORY_CONFIG_CANDIDATES {
        let path = dir.join(candidate);
        if path.is_file() {
            return Some(path);
        }
    }
    None
}
