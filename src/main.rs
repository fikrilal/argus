use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use std::env;

mod cli;
mod config;
mod git;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();
    let current_dir = env::current_dir()?;
    let argus_config = config::load_config(cli.config.as_deref(), &current_dir)?;

    if cli.verbose {
        println!("{}", "[argus] Verbose mode enabled".dimmed());
        println!(
            "{} project='{}' (version={})",
            "[argus] Config loaded:".dimmed(),
            argus_config.project.cyan(),
            argus_config.version
        );
    }

    match cli.command {
        cli::Commands::Audit(args) => {
            let active_branch = git::detect_current_branch(&current_dir)
                .await
                .unwrap_or_else(|_| "main".to_string());
            let branch_slug = git::slugify_branch_name(&active_branch);

            println!(
                "{} squad='{}' (branch='{}' [slug='{}'], base='{}')",
                "Running Argus audit for:".bold().cyan(),
                args.squad.yellow(),
                active_branch.green(),
                branch_slug.green(),
                args.base.as_deref().unwrap_or("auto-detect").yellow()
            );
        }
        cli::Commands::Init(args) => {
            cli::commands::init::run(&args)?;
        }
        cli::Commands::Personas(args) => match args.action {
            cli::PersonasSubcommand::List { squad } => {
                println!(
                    "{} (filter: {})",
                    "Available personas:".bold().blue(),
                    squad.as_deref().unwrap_or("none").yellow()
                );
            }
            cli::PersonasSubcommand::Show { name } => {
                println!("{} {}", "Persona details:".bold().blue(), name.yellow());
            }
        },
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
