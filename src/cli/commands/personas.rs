use anyhow::{Context, Result};
use colored::Colorize;

use crate::cli::args::{PersonasArgs, PersonasSubcommand};
use crate::config::ArgusConfig;
use crate::persona::{PersonaRegistry, PersonaSource};

/// Handles execution of the `argus personas` command suite (`list` and `show`).
pub fn run(args: &PersonasArgs, registry: &PersonaRegistry, config: &ArgusConfig) -> Result<()> {
    match &args.action {
        PersonasSubcommand::List { squad } => {
            let personas_to_list = if let Some(s) = squad {
                registry.get_squad(s, config)?
            } else {
                registry.all()
            };

            println!(
                "{} (total: {})",
                "Available personas:".bold().blue(),
                personas_to_list.len().to_string().yellow()
            );

            for p in personas_to_list {
                let source_label = match p.source {
                    PersonaSource::Builtin => "builtin".dimmed(),
                    PersonaSource::ProjectOverride(_) => "override".yellow(),
                };
                println!(
                    "  • {:<28} [{:<9}] {:<42} ({})",
                    p.name.bold(),
                    p.squad.cyan(),
                    p.title,
                    source_label
                );
            }
        }
        PersonasSubcommand::Show { name } => {
            let persona = registry
                .get(name)
                .with_context(|| format!("Persona '{name}' not found in registry"))?;

            println!("{} {}", "Persona:".bold().blue(), persona.name.yellow());
            println!("  Title:      {}", persona.title);
            println!("  Squad:      {}", persona.squad.cyan());
            println!("  Tier:       {:?}", persona.model_tier);
            println!("  Tools:      {}", persona.tools.join(", ").dimmed());
            println!("  Source:     {:?}", persona.source);
            println!(
                "\n{}\n{}",
                "System Prompt:".bold().green(),
                persona.system_prompt
            );
        }
    }

    Ok(())
}
