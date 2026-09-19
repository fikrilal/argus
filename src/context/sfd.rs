use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::ArgusConfig;

/// Represents an active System Functional Design (SFD) specification loaded into memory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SfdDocument {
    /// Filesystem path to the SFD file
    pub path: PathBuf,
    /// Raw Markdown content of the specification
    pub content: String,
    /// Extracted document title (e.g. from the first `# Title` heading)
    pub title: Option<String>,
}

impl SfdDocument {
    /// Creates an `SfdDocument` from a path and raw content, extracting the title if present.
    pub fn new(path: PathBuf, content: String) -> Self {
        let title = extract_title(&content);
        Self {
            path,
            content,
            title,
        }
    }
}

/// Discovers, reads, and validates the active Markdown SFD specification document.
///
/// Priority:
/// 1. `explicit_path` (supplied via CLI flag `--sfd <path>`)
/// 2. `config.sfd.active` (configured in `.argus/config.yaml`)
///
/// If neither is provided, returns `Ok(None)`, allowing audits to proceed without an SFD.
/// If a path is specified or configured but missing or empty, returns an explicit `Err`.
pub fn load_sfd(
    explicit_path: Option<&Path>,
    config: &ArgusConfig,
    project_root: &Path,
) -> Result<Option<SfdDocument>> {
    let resolved_path = if let Some(path) = explicit_path {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            project_root.join(path)
        }
    } else if let Some(ref active_file) = config.sfd.active {
        if active_file.is_absolute() {
            active_file.clone()
        } else {
            project_root.join(&config.sfd.dir).join(active_file)
        }
    } else {
        return Ok(None);
    };

    if !resolved_path.exists() {
        anyhow::bail!(
            "SFD specification file does not exist at '{}'",
            resolved_path.display()
        );
    }

    if !resolved_path.is_file() {
        anyhow::bail!(
            "SFD path is a directory, not a file: '{}'",
            resolved_path.display()
        );
    }

    let content = fs::read_to_string(&resolved_path).with_context(|| {
        format!(
            "Failed to read SFD specification file at '{}'",
            resolved_path.display()
        )
    })?;

    if content.trim().is_empty() {
        anyhow::bail!(
            "SFD specification file at '{}' is empty",
            resolved_path.display()
        );
    }

    Ok(Some(SfdDocument::new(resolved_path, content)))
}

/// Helper function to extract the first Markdown `# Heading` as the document title.
fn extract_title(content: &str) -> Option<String> {
    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(heading) = trimmed.strip_prefix("# ") {
            let title = heading.trim();
            if !title.is_empty() {
                return Some(title.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_extract_title_from_markdown() {
        let md = "# Stockist Kulakan Feature Spec\n\nSome introductory text...";
        assert_eq!(
            extract_title(md),
            Some("Stockist Kulakan Feature Spec".to_string())
        );

        let empty_md = "No heading here\nJust text";
        assert_eq!(extract_title(empty_md), None);
    }

    #[test]
    fn test_load_sfd_explicit_path() {
        let dir = tempdir().expect("tempdir");
        let sfd_path = dir.path().join("my-spec.md");
        fs::write(&sfd_path, "# My Feature\n\nDetails here").expect("write file");

        let config = ArgusConfig::default();
        let loaded = load_sfd(Some(&sfd_path), &config, dir.path())
            .expect("should load")
            .expect("should be some");

        assert_eq!(loaded.path, sfd_path);
        assert_eq!(loaded.title, Some("My Feature".to_string()));
        assert!(loaded.content.contains("Details here"));
    }

    #[test]
    fn test_load_sfd_configured_path() {
        let dir = tempdir().expect("tempdir");
        let sfd_dir = dir.path().join(".argus/context/sfd");
        fs::create_dir_all(&sfd_dir).expect("create sfd dir");

        let sfd_path = sfd_dir.join("configured-spec.md");
        fs::write(&sfd_path, "# Configured Spec\n\nRules here").expect("write file");

        let mut config = ArgusConfig::default();
        config.sfd.active = Some(PathBuf::from("configured-spec.md"));

        let loaded = load_sfd(None, &config, dir.path())
            .expect("should load")
            .expect("should be some");

        assert_eq!(loaded.path, sfd_path);
        assert_eq!(loaded.title, Some("Configured Spec".to_string()));
    }

    #[test]
    fn test_load_sfd_none_when_unconfigured() {
        let dir = tempdir().expect("tempdir");
        let config = ArgusConfig::default();

        let loaded = load_sfd(None, &config, dir.path()).expect("should succeed with None");
        assert_eq!(loaded, None);
    }

    #[test]
    fn test_load_sfd_missing_file_returns_error() {
        let dir = tempdir().expect("tempdir");
        let missing = dir.path().join("non-existent.md");
        let config = ArgusConfig::default();

        let result = load_sfd(Some(&missing), &config, dir.path());
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("does not exist"));
    }

    #[test]
    fn test_load_sfd_empty_file_returns_error() {
        let dir = tempdir().expect("tempdir");
        let empty_path = dir.path().join("empty.md");
        fs::write(&empty_path, "   \n\t  ").expect("write file");
        let config = ArgusConfig::default();

        let result = load_sfd(Some(&empty_path), &config, dir.path());
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("is empty"));
    }
}
