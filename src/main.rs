use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use std::env;

mod cli;
mod config;

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
            println!(
                "{} squad='{}' (base='{}')",
                "Running Argus audit for:".bold().cyan(),
                args.squad.yellow(),
                args.base.as_deref().unwrap_or("auto-detect").yellow()
            );
        }
        cli::Commands::Init(args) => {
            println!(
                "{} at {}",
                "Initializing Argus:".bold().green(),
                args.target_dir.display().to_string().yellow()
            );
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
