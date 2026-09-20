use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use std::env;

mod cli;
mod config;
mod context;
mod git;
mod persona;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let current_dir = env::current_dir()?;
    let argus_config = config::load_config(cli.config.as_deref(), &current_dir)?;
    let persona_registry = persona::PersonaRegistry::load(&current_dir)?;

    if cli.verbose {
        println!("{}", "[argus] Verbose mode enabled".dimmed());
        println!(
            "{} project='{}' (version={})",
            "[argus] Config loaded:".dimmed(),
            argus_config.project.cyan(),
            argus_config.version
        );
        println!(
            "{} {} registered",
            "[argus] Personas loaded:".dimmed(),
            persona_registry.len().to_string().cyan()
        );
    }

    match cli.command {
        cli::Commands::Audit(args) => {
            let active_branch = git::detect_current_branch(&current_dir)
                .await
                .unwrap_or_else(|_| "main".to_string());
            let branch_slug = git::slugify_branch_name(&active_branch);
            let sfd_doc = context::load_sfd(args.sfd.as_deref(), &argus_config, &current_dir)?;

            let bundle = context::TaskContextBundle::new(
                active_branch.clone(),
                args.base.clone(),
                args.staged,
                sfd_doc,
            );

            let squad_personas = persona_registry.get_squad(&args.squad, &argus_config)?;

            println!(
                "{} squad='{}' ({} agents, branch='{}' [slug='{}'], base='{}')",
                "Running Argus audit for:".bold().cyan(),
                args.squad.yellow(),
                squad_personas.len().to_string().bold().green(),
                bundle.branch.green(),
                branch_slug.green(),
                args.base.as_deref().unwrap_or("auto-detect").yellow()
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
            for p in &squad_personas {
                let source_label = match p.source {
                    persona::PersonaSource::Builtin => "builtin".dimmed(),
                    persona::PersonaSource::ProjectOverride(_) => "override".yellow(),
                };
                println!(
                    "  • {:<28} [{}] ({})",
                    p.name.bold(),
                    p.squad.cyan(),
                    source_label
                );
            }
        }
        cli::Commands::Init(args) => {
            cli::commands::init::run(&args)?;
        }
        cli::Commands::Personas(args) => {
            cli::commands::personas::run(&args, &persona_registry, &argus_config)?;
        }
        cli::Commands::Resume(args) => {
            println!(
                "{} (persona: {})",
                "Resuming agent session:".bold().magenta(),
                args.persona
                    .as_deref()
                    .unwrap_or("interactive picker")
                    .yellow()
            );
        }
    }

    Ok(())
}
