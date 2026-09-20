use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use std::env;

mod cli;
mod config;
mod context;
mod git;
mod persona;
mod runner;
mod synthesis;

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
            cli::commands::audit::run(&args, &argus_config, &persona_registry, &current_dir)
                .await?;
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
