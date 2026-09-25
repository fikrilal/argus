use anyhow::{Context, Result};
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
    let current_dir =
        env::current_dir().context("Failed to determine current working directory")?;

    match cli.command {
        cli::Commands::Init(args) => {
            cli::commands::init::run(&args).await?;
        }
        cli::Commands::Audit(args) => {
            let argus_config = config::load_config(cli.config.as_deref(), &current_dir)?;
            let persona_registry = persona::PersonaRegistry::load(&current_dir)?;
            log_verbose_context(cli.verbose, &argus_config, &persona_registry);
            cli::commands::audit::run(&args, &argus_config, &persona_registry, &current_dir)
                .await?;
        }
        cli::Commands::Personas(args) => {
            let argus_config = config::load_config(cli.config.as_deref(), &current_dir)?;
            let persona_registry = persona::PersonaRegistry::load(&current_dir)?;
            log_verbose_context(cli.verbose, &argus_config, &persona_registry);
            cli::commands::personas::run(&args, &persona_registry, &argus_config)?;
        }
        cli::Commands::Resume(args) => {
            let persona_registry = persona::PersonaRegistry::load(&current_dir)?;
            cli::commands::resume::run(&args, &persona_registry, &current_dir).await?;
        }
    }

    Ok(())
}

fn log_verbose_context(
    verbose: bool,
    config: &config::ArgusConfig,
    registry: &persona::PersonaRegistry,
) {
    if verbose {
        println!("{}", "[argus] Verbose mode enabled".dimmed());
        println!(
            "{} project='{}' (version={})",
            "[argus] Config loaded:".dimmed(),
            config.project.cyan(),
            config.version
        );
        println!(
            "{} {} registered",
            "[argus] Personas loaded:".dimmed(),
            registry.len().to_string().cyan()
        );
    }
}
