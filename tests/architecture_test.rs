#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

use std::path::Path;
use walkdir::WalkDir;

const FORBIDDEN_GENERIC_NAMES: &[&str] = &["helper", "helpers", "manager", "utils"];

#[test]
fn test_no_generic_file_names() {
    let src_dir = Path::new("src");
    if !src_dir.exists() {
        return;
    }

    let mut violations = Vec::new();

    for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
        if let Some(file_stem) = entry
            .path()
            .is_file()
            .then(|| entry.path().file_stem().and_then(|s| s.to_str()))
            .flatten()
        {
            let lower = file_stem.to_lowercase();
            for forbidden in FORBIDDEN_GENERIC_NAMES {
                if lower == *forbidden
                    || lower.ends_with(&format!("_{}", forbidden))
                    || lower.starts_with(&format!("{}_", forbidden))
                {
                    violations.push(format!(
                        "{}: generic name '{}' hides ownership. Use a domain-specific name.",
                        entry.path().display(),
                        forbidden
                    ));
                }
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Architecture violations detected:\n{}",
        violations.join("\n")
    );
}

#[test]
fn test_no_unsafe_code() {
    let src_dir = Path::new("src");
    if !src_dir.exists() {
        return;
    }

    let mut violations = Vec::new();

    for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().is_some_and(|ext| ext == "rs") {
            let content = match std::fs::read_to_string(entry.path()) {
                Ok(c) => c,
                Err(e) => {
                    panic!("Failed to read {}: {}", entry.path().display(), e);
                }
            };

            // Check for unsafe keyword usage outside comments
            for (line_idx, line) in content.lines().enumerate() {
                let trimmed = line.trim();
                if trimmed.starts_with("//")
                    || trimmed.starts_with("/*")
                    || trimmed.starts_with('*')
                {
                    continue;
                }
                if line.contains("unsafe ") || line.contains("unsafe{") {
                    violations.push(format!(
                        "{}:{}: unsafe code is strictly forbidden in Argus.",
                        entry.path().display(),
                        line_idx + 1
                    ));
                }
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Unsafe code detected in production modules:\n{}",
        violations.join("\n")
    );
}

#[test]
fn test_no_blocking_std_process_in_runner() {
    let runner_dir = Path::new("src/runner");
    if !runner_dir.exists() {
        return;
    }

    let mut violations = Vec::new();

    for entry in WalkDir::new(runner_dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().is_some_and(|ext| ext == "rs") {
            let content = match std::fs::read_to_string(entry.path()) {
                Ok(c) => c,
                Err(e) => {
                    panic!("Failed to read {}: {}", entry.path().display(), e);
                }
            };

            for (line_idx, line) in content.lines().enumerate() {
                let trimmed = line.trim();
                if trimmed.starts_with("//") {
                    continue;
                }
                if line.contains("std::process::Command") {
                    violations.push(format!(
                        "{}:{}: runner must use tokio::process::Command for non-blocking asynchronous execution.",
                        entry.path().display(),
                        line_idx + 1
                    ));
                }
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Blocking std::process::Command detected in runner:\n{}",
        violations.join("\n")
    );
}
