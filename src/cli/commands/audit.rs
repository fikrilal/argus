use anyhow::Result;
use colored::Colorize;
use std::path::Path;

use crate::cli::args::AuditArgs;
use crate::config::ArgusConfig;
use crate::context::{self, TaskContextBundle};
use crate::git;
use crate::persona::{ModelTier, Persona, PersonaRegistry, PersonaSource};
use crate::runner::{self, AgentExecutionPlan, SwarmPool, SwarmProgressTracker};
use crate::synthesis::{self, AgentReport, AuditSynthesis};

/// Executes the `argus audit` workflow.
pub async fn run(
    args: &AuditArgs,
    config: &ArgusConfig,
    registry: &PersonaRegistry,
    current_dir: &Path,
) -> Result<()> {
    if runner::is_recursive_audit() {
        anyhow::bail!(
            "Recursive audit blocked: cannot run 'argus audit' from within an active Argus agent session."
        );
    }

    let active_branch = git::detect_current_branch(current_dir)
        .await
        .unwrap_or_else(|_| "main".to_string());
    let branch_slug = git::slugify_branch_name(&active_branch);
    let sfd_doc = context::load_sfd(args.sfd.as_deref(), config, current_dir)?;

    let bundle = TaskContextBundle::new(
        active_branch.clone(),
        args.base.clone(),
        args.staged,
        args.full,
        args.path.clone(),
        sfd_doc,
    );

    let squad_personas = registry.get_squad(&args.squad, config)?;
    print_audit_header(args, &bundle, &branch_slug, &squad_personas);

    let plans = build_execution_plans(&squad_personas, &branch_slug, &bundle, config);

    println!();
    let tracker = SwarmProgressTracker::new();
    let pool =
        SwarmPool::new(args.concurrency).with_timeout(std::time::Duration::from_secs(args.timeout));
    let results = pool.execute_all(plans, current_dir, Some(&tracker)).await?;

    let failed_agents: Vec<&runner::AgentRunResult> =
        results.iter().filter(|r| !r.is_success()).collect();
    let detailed_failures: Vec<&runner::AgentRunResult> = failed_agents
        .into_iter()
        .filter(|f| {
            let err = f.stderr.trim();
            !err.is_empty()
                && !err.starts_with("Agent execution timed out")
                && !err.starts_with("process exited with code")
        })
        .collect();

    if !detailed_failures.is_empty() {
        eprintln!("\n{}", "Failure diagnostics:".bold().yellow());
        for f in detailed_failures {
            eprintln!(
                "  • {:<30} {}",
                f.persona_name.yellow(),
                f.stderr.trim().dimmed()
            );
        }
    }

    let reports: Vec<AgentReport> = results
        .into_iter()
        .map(|r| {
            let is_success = r.is_success();
            let raw_output = if is_success { r.stdout } else { r.stderr };
            AgentReport {
                persona_name: r.persona_name,
                raw_output,
                is_success,
            }
        })
        .collect();

    let synthesis = synthesis::synthesize_reports(&reports);

    let dashboard = synthesis::render_terminal_dashboard(&synthesis);
    print!("{dashboard}");

    save_audit_report(&synthesis, &bundle, &args.squad, current_dir);

    if !synthesis.is_passed() {
        anyhow::bail!(
            "Audit identified {} blocker(s) and {} major defect(s). Action required before QA handoff.",
            synthesis.blocker_count,
            synthesis.major_count
        );
    }

    Ok(())
}

fn print_audit_header(
    args: &AuditArgs,
    bundle: &TaskContextBundle,
    branch_slug: &str,
    squad_personas: &[&Persona],
) {
    let mode_desc = if let Some(ref p) = args.path {
        format!("path='{}'", p.display())
    } else if args.full {
        "full-codebase".to_string()
    } else if args.staged {
        "staged-only".to_string()
    } else {
        format!("base='{}'", args.base.as_deref().unwrap_or("working-tree"))
    };

    println!(
        "{} squad='{}' ({} agents, branch='{}' [slug='{}'], {}, concurrency={})",
        "Running Argus audit for:".bold().cyan(),
        args.squad.yellow(),
        squad_personas.len().to_string().bold().green(),
        bundle.branch.green(),
        branch_slug.green(),
        mode_desc.yellow(),
        args.concurrency.to_string().yellow()
    );

    if let Some(ref sfd) = bundle.sfd {
        println!(
            "{} '{}' ({})",
            "Active SFD:".bold().green(),
            sfd.title.as_deref().unwrap_or("Untitled Spec").yellow(),
            sfd.path.display().to_string().dimmed()
        );
    }

    println!("{}", "Deploying agent squad:".bold().blue());
    for p in squad_personas {
        let source_label = match p.source {
            PersonaSource::Builtin => "builtin".dimmed(),
            PersonaSource::ProjectOverride(_) => "override".yellow(),
        };
        println!(
            "  • {:<28} [{}] ({})",
            p.name.bold(),
            p.squad.cyan(),
            source_label
        );
    }
}

fn build_execution_plans(
    squad_personas: &[&Persona],
    branch_slug: &str,
    bundle: &TaskContextBundle,
    config: &ArgusConfig,
) -> Vec<AgentExecutionPlan> {
    squad_personas
        .iter()
        .map(|p| {
            let session_name = runner::format_session_name(branch_slug, &p.name);
            let task_payload = context::build_agent_prompt(bundle, &p.name);
            let model = match p.model_tier {
                ModelTier::Fast => config.models.fast.clone(),
                ModelTier::Standard => config.models.standard.clone(),
                ModelTier::Deep => config.models.deep.clone(),
            };

            AgentExecutionPlan {
                persona_name: p.name.clone(),
                session_name,
                system_prompt: p.system_prompt.clone(),
                task_payload,
                model,
                tools: p.tools.clone(),
            }
        })
        .collect()
}

fn save_audit_report(
    synthesis: &AuditSynthesis,
    bundle: &TaskContextBundle,
    squad: &str,
    current_dir: &Path,
) {
    let reports_dir = current_dir.join(".argus").join("reports");
    let (sfd_title, sfd_path) = match bundle.sfd {
        Some(ref sfd) => (sfd.title.as_deref(), sfd.path.to_str()),
        None => (None, None),
    };

    let md_report =
        synthesis::generate_markdown_report(synthesis, &bundle.branch, squad, sfd_title, sfd_path);

    match synthesis::write_markdown_report(&md_report, &reports_dir) {
        Ok(path) => {
            println!(
                "{} Saved persistent audit report to: {}",
                "✔".bold().green(),
                path.display().to_string().cyan()
            );
        }
        Err(err) => {
            eprintln!(
                "{} Failed to write persistent audit report to '{}': {err:#}",
                "⚠".bold().yellow(),
                reports_dir.display()
            );
        }
    }
}
