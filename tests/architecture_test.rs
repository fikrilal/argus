#![allow(clippy::panic, clippy::unwrap_used, clippy::expect_used)]

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const FORBIDDEN_GENERIC_NAMES: &[&str] = &["helper", "helpers", "manager", "utils"];

/// 1. Mechanical check: Disallow generic dumping-ground file or directory names.
#[test]
fn test_no_generic_file_names() {
    let src_dir = Path::new("src");
    if !src_dir.exists() {
        return;
    }

    let mut violations = Vec::new();

    for entry in WalkDir::new(src_dir).into_iter().filter_map(Result::ok) {
        if let Some(file_stem) = entry
            .path()
            .is_file()
            .then(|| entry.path().file_stem().and_then(|s| s.to_str()))
            .flatten()
        {
            let lower = file_stem.to_lowercase();
            for forbidden in FORBIDDEN_GENERIC_NAMES {
                if lower == *forbidden
                    || lower.ends_with(&format!("_{forbidden}"))
                    || lower.starts_with(&format!("{forbidden}_"))
                {
                    violations.push(format!(
                        "{}: generic name '{forbidden}' hides ownership. Use a domain-specific name.",
                        entry.path().display()
                    ));
                }
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Generic naming violations detected:\n{}",
        violations.join("\n")
    );
}

/// 2. Mechanical check: Disallow `unsafe` code anywhere in `src/`.
#[test]
fn test_no_unsafe_code() {
    let src_dir = Path::new("src");
    if !src_dir.exists() {
        return;
    }

    let mut violations = Vec::new();

    for entry in WalkDir::new(src_dir).into_iter().filter_map(Result::ok) {
        if entry.path().extension().is_some_and(|ext| ext == "rs") {
            let content = match fs::read_to_string(entry.path()) {
                Ok(c) => c,
                Err(e) => {
                    panic!("Failed to read {}: {}", entry.path().display(), e);
                }
            };

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

/// 3. Mechanical check: Enforce non-blocking async execution (`std::process::Command` is forbidden).
#[test]
fn test_no_blocking_std_process_across_src() {
    let src_dir = Path::new("src");
    if !src_dir.exists() {
        return;
    }

    let mut violations = Vec::new();

    for entry in WalkDir::new(src_dir).into_iter().filter_map(Result::ok) {
        if entry.path().extension().is_some_and(|ext| ext == "rs") {
            let content = match fs::read_to_string(entry.path()) {
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
                        "{}:{}: synchronous std::process::Command is forbidden. Use tokio::process::Command for non-blocking asynchronous execution.",
                        entry.path().display(),
                        line_idx + 1
                    ));
                }
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Blocking std::process::Command detected in src/:\n{}",
        violations.join("\n")
    );
}

/// 4. Mechanical check: Architecture Layer Boundary & Unidirectional Dependency Rules.
///
/// Rules:
/// - `config`: Leaf module; may not import `cli`, `runner`, `git`, `context`, `persona`, `synthesis`, or `oracles`.
/// - `git`: Leaf module; may not import `cli`, `runner`, `persona`, `context`, `synthesis`, or `oracles`.
/// - `persona`: Core domain; may import `config`, but may not import `cli`, `runner`, `git`, `context`, or `synthesis`.
/// - `context`: Ingestion; may import `config`, but may not import `cli`, `runner`, `git`, `synthesis`, or `oracles`.
/// - `runner`: Subprocess engine; may not import `cli` or `synthesis`.
/// - `synthesis`: Reporting; may not import `cli` or `runner`.
/// - `cli`: Presentation edge; orchestrates lower layers.
#[test]
fn test_layer_dependency_boundaries() {
    let mut layer_forbidden_imports: HashMap<&'static str, Vec<&'static str>> = HashMap::new();

    layer_forbidden_imports.insert(
        "config",
        vec![
            "crate::cli",
            "crate::runner",
            "crate::git",
            "crate::context",
            "crate::persona",
            "crate::synthesis",
            "crate::oracles",
        ],
    );

    layer_forbidden_imports.insert(
        "git",
        vec![
            "crate::cli",
            "crate::runner",
            "crate::persona",
            "crate::context",
            "crate::synthesis",
            "crate::oracles",
        ],
    );

    layer_forbidden_imports.insert(
        "persona",
        vec![
            "crate::cli",
            "crate::runner",
            "crate::git",
            "crate::context",
            "crate::synthesis",
        ],
    );

    layer_forbidden_imports.insert(
        "context",
        vec![
            "crate::cli",
            "crate::runner",
            "crate::git",
            "crate::synthesis",
            "crate::oracles",
        ],
    );

    layer_forbidden_imports.insert("runner", vec!["crate::cli", "crate::synthesis"]);

    layer_forbidden_imports.insert("synthesis", vec!["crate::cli", "crate::runner"]);

    let mut violations = Vec::new();

    for (layer, forbidden_list) in &layer_forbidden_imports {
        let layer_dir = PathBuf::from(format!("src/{layer}"));
        if !layer_dir.exists() {
            continue;
        }

        for entry in WalkDir::new(&layer_dir).into_iter().filter_map(Result::ok) {
            if entry.path().extension().is_some_and(|ext| ext == "rs") {
                let content = match fs::read_to_string(entry.path()) {
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

                    for forbidden in forbidden_list {
                        if line.contains(forbidden) {
                            violations.push(format!(
                                "{}:{}: architectural layer boundary violation: '{}' must not depend on '{}'.",
                                entry.path().display(),
                                line_idx + 1,
                                layer,
                                forbidden
                            ));
                        }
                    }
                }
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Architectural layer boundary violations detected:\n{}",
        violations.join("\n")
    );
}

/// 5. Mechanical check: Console Output (`println!`) Quarantine.
///
/// Internal library and engine modules (`src/config/`, `src/context/`, `src/git/`,
/// `src/persona/`, `src/runner/`, `src/synthesis/`) must NEVER call `println!` or `eprintln!`.
/// Console output is strictly quarantined to `src/cli/` and `src/main.rs`.
#[test]
fn test_no_unquarantined_println() {
    let src_dir = Path::new("src");
    if !src_dir.exists() {
        return;
    }

    let mut violations = Vec::new();

    for entry in WalkDir::new(src_dir).into_iter().filter_map(Result::ok) {
        let path = entry.path();
        if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }

        // Allowed files: src/main.rs and anything inside src/cli/
        let rel_path = path.strip_prefix(src_dir).unwrap_or(path);
        if rel_path == Path::new("main.rs") || rel_path.starts_with("cli") {
            continue;
        }

        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                panic!("Failed to read {}: {}", path.display(), e);
            }
        };

        for (line_idx, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") {
                continue;
            }

            // Exclude unit test modules in files
            if content[..content.find(line).unwrap_or(0)].contains("#[cfg(test)]") {
                continue;
            }

            if line.contains("println!")
                || line.contains("eprintln!")
                || line.contains("print!")
                || line.contains("eprint!")
            {
                violations.push(format!(
                    "{}:{}: console output macro is forbidden in internal library modules. Quarantine console UI to src/cli/ or return structured results.",
                    path.display(),
                    line_idx + 1
                ));
            }
        }
    }

    assert!(
        violations.is_empty(),
        "Unquarantined console output detected in library code:\n{}",
        violations.join("\n")
    );
}

/// 6. Mechanical check: Bidirectional Project Map Drift Verification.
///
/// Ensures every module directory in `src/` is explicitly documented in `AGENTS.md`
/// under `## Architecture Map`, and every module documented in `AGENTS.md` exists on disk.
#[test]
fn test_project_map_drift() {
    let agents_md_path = Path::new("AGENTS.md");
    assert!(
        agents_md_path.exists(),
        "AGENTS.md must exist at repository root"
    );

    let content = fs::read_to_string(agents_md_path).expect("should read AGENTS.md");
    let src_dir = Path::new("src");

    let entries = fs::read_dir(src_dir).expect("should read src/ directory");
    let mut actual_dirs = std::collections::HashSet::new();
    let mut missing_from_agents_md = Vec::new();

    for entry in entries.filter_map(Result::ok) {
        if let Some(dir_name) = entry
            .path()
            .is_dir()
            .then(|| entry.file_name().into_string().ok())
            .flatten()
        {
            let token = format!("├── {dir_name}/");
            let alt_token = format!("└── {dir_name}/");
            if !content.contains(&token) && !content.contains(&alt_token) {
                missing_from_agents_md.push(dir_name.clone());
            }
            actual_dirs.insert(dir_name);
        }
    }

    assert!(
        missing_from_agents_md.is_empty(),
        "Directories in src/ missing from AGENTS.md Architecture Map:\n{}",
        missing_from_agents_md.join("\n")
    );

    // Bidirectional check: ensure modules documented in AGENTS.md actually exist on disk
    let mut missing_from_disk = Vec::new();
    let mut in_arch_map = false;

    for line in content.lines() {
        if line.contains("## Architecture Map") {
            in_arch_map = true;
            continue;
        }
        if in_arch_map && line.starts_with("## ") {
            break;
        }
        if in_arch_map {
            let trimmed = line.trim();
            let is_tree_entry = trimmed.starts_with("├── ") || trimmed.starts_with("└── ");
            if is_tree_entry {
                let mod_name = trimmed
                    .split_once(' ')
                    .and_then(|(_, rest)| rest.split_whitespace().next())
                    .and_then(|token| token.strip_suffix('/'));

                if let Some(missing_mod) = mod_name.filter(|name| !actual_dirs.contains(*name)) {
                    missing_from_disk.push(missing_mod.to_string());
                }
            }
        }
    }

    assert!(
        missing_from_disk.is_empty(),
        "Modules documented in AGENTS.md Architecture Map do not exist in src/:\n{}",
        missing_from_disk.join("\n")
    );
}
