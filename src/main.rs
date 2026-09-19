use anyhow::Result;
use clap::Parser;
use colored::Colorize;

mod cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = cli::Cli::parse();

    if cli.verbose {
        println!("{}", "[argus] Verbose mode enabled".dimmed());
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
