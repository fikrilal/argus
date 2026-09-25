use anyhow::{Context, Result};
use colored::Colorize;
use std::io::{self, Write};
use std::path::Path;

use crate::cli::args::ResumeArgs;
use crate::git;
use crate::persona::PersonaRegistry;
use crate::runner;

/// Pure function resolving the target Pi session name from arguments or user selection.
pub fn resolve_resume_session_name(
    args: &ResumeArgs,
    active_branch_slug: &str,
    registry: &PersonaRegistry,
    interactive_input: Option<&str>,
) -> Result<String> {
    let branch_slug = if let Some(ref b) = args.branch {
        git::slugify_branch_name(b)
    } else {
        active_branch_slug.to_string()
    };

    if let Some(ref persona_name) = args.persona {
        if registry.get(persona_name).is_none() {
            anyhow::bail!(
                "Persona '{persona_name}' not found in registry. Run 'argus personas list' to view available personas."
            );
        }
        return Ok(runner::format_session_name(&branch_slug, persona_name));
    }

    if let Some(input) = interactive_input {
        let trimmed = input.trim();
        if let Ok(idx) = trimmed.parse::<usize>() {
            let all = registry.all();
            if idx >= 1 && idx <= all.len() {
                return Ok(runner::format_session_name(
                    &branch_slug,
                    &all[idx - 1].name,
                ));
            }
            anyhow::bail!(
                "Invalid selection index '{idx}'. Please select between 1 and {}.",
                all.len()
            );
        }

        if registry.get(trimmed).is_some() {
            return Ok(runner::format_session_name(&branch_slug, trimmed));
        }

        anyhow::bail!("Unknown persona '{trimmed}' selected.");
    }

    anyhow::bail!("No persona specified. Provide --persona <name> or select interactively.");
}

/// Executes the `argus resume` command.
pub async fn run(args: &ResumeArgs, registry: &PersonaRegistry, current_dir: &Path) -> Result<()> {
    let active_branch = git::detect_current_branch(current_dir)
        .await
        .unwrap_or_else(|_| "main".to_string());
    let active_branch_slug = git::slugify_branch_name(&active_branch);

    let session_name = if args.persona.is_some() {
        resolve_resume_session_name(args, &active_branch_slug, registry, None)?
    } else {
        let target_branch = args
            .branch
            .as_deref()
            .map_or_else(|| active_branch_slug.clone(), git::slugify_branch_name);

        println!(
            "{} on branch '{}':",
            "Available Argus personas to resume".bold().blue(),
            target_branch.green()
        );

        let all = registry.all();
        for (idx, p) in all.iter().enumerate() {
            let num = idx + 1;
            println!("  [{num:>2}] {:<28} [{}]", p.name.bold(), p.squad.cyan());
        }

        print!(
            "\n{}",
            format!("Select persona [1-{} or name]: ", all.len())
                .bold()
                .yellow()
        );
        io::stdout()
            .flush()
            .context("Failed to flush stdout prompt during interactive session resume")?;

        let mut user_input = String::new();
        let mut reader = tokio::io::BufReader::new(tokio::io::stdin());
        tokio::io::AsyncBufReadExt::read_line(&mut reader, &mut user_input)
            .await
            .context("Failed to read user selection from stdin during interactive resume")?;

        resolve_resume_session_name(args, &active_branch_slug, registry, Some(&user_input))?
    };

    let pi_bin = std::env::var("ARGUS_PI_BIN").unwrap_or_else(|_| "pi".to_string());

    println!(
        "\n{} '{}' via {}...\n",
        "Resuming interactive session:".bold().green(),
        session_name.cyan(),
        pi_bin.dimmed()
    );

    let status = runner::resume_interactive_session(&session_name, current_dir).await?;

    if !status.success() {
        let code = status.code().unwrap_or(-1);
        anyhow::bail!("Pi session exited with status code {code}");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_resolve_resume_session_name_direct() {
        let dir = tempdir().expect("tempdir");
        let registry = PersonaRegistry::load(dir.path()).expect("registry");

        let args = ResumeArgs {
            persona: Some("stock-ledger-auditor".to_string()),
            branch: None,
        };

        let session = resolve_resume_session_name(&args, "DEV-AFM-AUTH_MIGRATION", &registry, None)
            .expect("should resolve");
        assert_eq!(session, "argus/DEV-AFM-AUTH_MIGRATION/stock-ledger-auditor");
    }

    #[test]
    fn test_resolve_resume_session_name_with_branch_override() {
        let dir = tempdir().expect("tempdir");
        let registry = PersonaRegistry::load(dir.path()).expect("registry");

        let args = ResumeArgs {
            persona: Some("id-regulatory-sentinel".to_string()),
            branch: Some("feature/custom-branch".to_string()),
        };

        let session =
            resolve_resume_session_name(&args, "main", &registry, None).expect("should resolve");
        assert_eq!(
            session,
            "argus/feature-custom-branch/id-regulatory-sentinel"
        );
    }

    #[test]
    fn test_resolve_resume_session_name_interactive_by_index() {
        let dir = tempdir().expect("tempdir");
        let registry = PersonaRegistry::load(dir.path()).expect("registry");

        let args = ResumeArgs {
            persona: None,
            branch: None,
        };

        let all = registry.all();
        let first_persona_name = &all[0].name;

        let session = resolve_resume_session_name(&args, "main", &registry, Some("1\n"))
            .expect("should resolve by index");
        assert_eq!(session, format!("argus/main/{first_persona_name}"));
    }

    #[test]
    fn test_resolve_resume_session_name_unknown_persona_error() {
        let dir = tempdir().expect("tempdir");
        let registry = PersonaRegistry::load(dir.path()).expect("registry");

        let args = ResumeArgs {
            persona: Some("non-existent-agent".to_string()),
            branch: None,
        };

        let result = resolve_resume_session_name(&args, "main", &registry, None);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Persona 'non-existent-agent' not found")
        );
    }
}
