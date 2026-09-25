use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::config::ArgusConfig;
use crate::persona::builtin::get_builtin_personas;
use crate::persona::frontmatter::{Persona, PersonaSource, parse_persona_markdown};

const PERSONA_CANDIDATE_DIRS: &[&str] = &[".argus/personas", ".swarm/personas"];

/// Central registry managing all agent personas with two-tier resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersonaRegistry {
    personas: HashMap<String, Persona>,
}

impl PersonaRegistry {
    /// Loads the persona registry with two-tier resolution:
    /// 1. Seeds all 12 compiled-in Tier-1 base personas.
    /// 2. Scans `<project_root>/.argus/personas/` and overrides or adds personas.
    pub fn load(project_root: &Path) -> Result<Self> {
        let mut map = HashMap::new();

        // 1. Seed built-in Tier-1 personas
        let builtins = get_builtin_personas()?;
        for persona in builtins {
            map.insert(persona.name.clone(), persona);
        }

        // 2. Scan project candidate directories for overrides and extensions.
        // Stop at the first candidate directory found to establish clean priority (.argus > .swarm).
        for candidate in PERSONA_CANDIDATE_DIRS {
            let candidate_dir = project_root.join(candidate);
            if candidate_dir.is_dir() {
                load_personas_from_dir(&candidate_dir, &mut map)?;
                break;
            }
        }

        Ok(Self { personas: map })
    }

    /// Looks up a persona by its exact name.
    pub fn get(&self, name: &str) -> Option<&Persona> {
        self.personas.get(name)
    }

    /// Returns all registered personas sorted alphabetically by name.
    pub fn all(&self) -> Vec<&Persona> {
        let mut list: Vec<&Persona> = self.personas.values().collect();
        list.sort_by(|a, b| a.name.cmp(&b.name));
        list
    }

    /// Returns the total number of registered personas.
    pub fn len(&self) -> usize {
        self.personas.len()
    }

    /// Returns true if no personas are registered.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.personas.is_empty()
    }

    /// Resolves and returns the list of personas belonging to a squad.
    ///
    /// Resolution rules:
    /// - Checks `config.squads` for `squad_name`.
    /// - If found, collects each persona named in the squad list.
    /// - If `squad_name` is `"all"` and not defined in `config.squads`, returns all non-synthesis personas.
    /// - Fails if a requested persona name is missing from the registry.
    pub fn get_squad(&self, squad_name: &str, config: &ArgusConfig) -> Result<Vec<&Persona>> {
        if squad_name == "all" {
            if let Some(names) = config.squads.get("all") {
                // If custom personas were registered beyond the 12 builtins and the squad is at default count,
                // dynamically return all worker personas so custom additions are not silently omitted.
                if self.len() > 12 && names.len() == 11 {
                    let workers: Vec<&Persona> = self
                        .all()
                        .into_iter()
                        .filter(|p| p.squad != "synthesis" && p.name != "lead-qa-synthesizer")
                        .collect();
                    return Ok(workers);
                }

                let mut resolved = Vec::with_capacity(names.len());
                for name in names {
                    let persona = self.get(name).ok_or_else(|| {
                        anyhow::anyhow!(
                            "Squad '{squad_name}' references persona '{name}', which is not registered in the registry"
                        )
                    })?;
                    resolved.push(persona);
                }
                return Ok(resolved);
            }

            let workers: Vec<&Persona> = self
                .all()
                .into_iter()
                .filter(|p| p.squad != "synthesis" && p.name != "lead-qa-synthesizer")
                .collect();
            return Ok(workers);
        }

        if let Some(names) = config.squads.get(squad_name) {
            let mut resolved = Vec::with_capacity(names.len());
            for name in names {
                let persona = self.get(name).ok_or_else(|| {
                    anyhow::anyhow!(
                        "Squad '{squad_name}' references persona '{name}', which is not registered in the registry"
                    )
                })?;
                resolved.push(persona);
            }
            return Ok(resolved);
        }

        let mut available_squads: Vec<String> = config.squads.keys().cloned().collect();
        available_squads.sort();

        anyhow::bail!(
            "Unknown squad '{}'. Available squads in config: {}",
            squad_name,
            available_squads.join(", ")
        );
    }
}

fn load_personas_from_dir(dir: &Path, map: &mut HashMap<String, Persona>) -> Result<()> {
    let entries = fs::read_dir(dir)
        .with_context(|| format!("Failed to read persona directory at '{}'", dir.display()))?;

    for entry in entries {
        let entry = entry
            .with_context(|| format!("Failed to read directory entry in '{}'", dir.display()))?;
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "md") {
            let content = fs::read_to_string(&path)
                .with_context(|| format!("Failed to read persona file at '{}'", path.display()))?;

            let persona =
                parse_persona_markdown(&content, PersonaSource::ProjectOverride(path.clone()))
                    .with_context(|| {
                        format!("Failed to parse persona file at '{}'", path.display())
                    })?;

            map.insert(persona.name.clone(), persona);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_registry_load_default_builtins() {
        let dir = tempdir().expect("tempdir");
        let registry = PersonaRegistry::load(dir.path()).expect("should load registry");

        assert_eq!(registry.len(), 12);
        assert!(!registry.is_empty());

        let form_saboteur = registry.get("form-boundary-saboteur").unwrap();
        assert_eq!(form_saboteur.name, "form-boundary-saboteur");
        assert_eq!(form_saboteur.source, PersonaSource::Builtin);

        let stock_auditor = registry.get("stock-ledger-auditor").unwrap();
        assert_eq!(stock_auditor.name, "stock-ledger-auditor");
        assert_eq!(stock_auditor.source, PersonaSource::Builtin);
    }

    #[test]
    fn test_registry_override_builtin_persona() {
        let dir = tempdir().expect("tempdir");
        let personas_dir = dir.path().join(".argus/personas");
        fs::create_dir_all(&personas_dir).expect("create dir");

        let override_file = personas_dir.join("stock-ledger-auditor.md");
        let override_content = r"---
name: stock-ledger-auditor
title: Custom Kalbe Drift Ledger Auditor
squad: state
model_tier: deep
tools: read, grep, find, ls, bash
---

Customized prompt checking SimplidotInvoiceOutboxDao specifically.
";
        fs::write(&override_file, override_content).expect("write file");

        let registry = PersonaRegistry::load(dir.path()).expect("should load registry");
        assert_eq!(registry.len(), 12);

        let auditor = registry.get("stock-ledger-auditor").unwrap();
        assert_eq!(auditor.title, "Custom Kalbe Drift Ledger Auditor");
        assert!(
            auditor
                .system_prompt
                .contains("SimplidotInvoiceOutboxDao specifically")
        );
        assert_eq!(
            auditor.source,
            PersonaSource::ProjectOverride(override_file)
        );
    }

    #[test]
    fn test_registry_novel_custom_persona() {
        let dir = tempdir().expect("tempdir");
        let personas_dir = dir.path().join(".argus/personas");
        fs::create_dir_all(&personas_dir).expect("create dir");

        let custom_file = personas_dir.join("pci-compliance-sentinel.md");
        let custom_content = r"---
name: pci-compliance-sentinel
title: PCI-DSS Compliance Sentinel
squad: security
model_tier: standard
tools: read, grep, find, ls, bash
---

Audit payment card numbers.
";
        fs::write(&custom_file, custom_content).expect("write file");

        let registry = PersonaRegistry::load(dir.path()).expect("should load registry");
        assert_eq!(registry.len(), 13);

        let pci = registry.get("pci-compliance-sentinel").unwrap();
        assert_eq!(pci.title, "PCI-DSS Compliance Sentinel");
        assert_eq!(pci.squad, "security");
    }

    #[test]
    fn test_registry_get_squad_forms() {
        let dir = tempdir().expect("tempdir");
        let registry = PersonaRegistry::load(dir.path()).expect("should load registry");
        let config = ArgusConfig::default();

        let forms_squad = registry.get_squad("forms", &config).unwrap();
        assert_eq!(forms_squad.len(), 3);
        let names: Vec<&str> = forms_squad.iter().map(|p| p.name.as_str()).collect();
        assert!(names.contains(&"id-regulatory-sentinel"));
        assert!(names.contains(&"form-boundary-saboteur"));
        assert!(names.contains(&"cascade-dropdown-glitcher"));
    }

    #[test]
    fn test_registry_unknown_squad_error() {
        let dir = tempdir().expect("tempdir");
        let registry = PersonaRegistry::load(dir.path()).expect("should load registry");
        let config = ArgusConfig::default();

        let result = registry.get_squad("unknown-squad", &config);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("Unknown squad 'unknown-squad'"));
        assert!(err.contains("Available squads in config"));
    }

    #[test]
    fn test_registry_argus_over_swarm_priority() {
        let dir = tempdir().expect("tempdir");
        let argus_personas = dir.path().join(".argus/personas");
        let swarm_personas = dir.path().join(".swarm/personas");
        fs::create_dir_all(&argus_personas).expect("create argus");
        fs::create_dir_all(&swarm_personas).expect("create swarm");

        let argus_file = argus_personas.join("stock-ledger-auditor.md");
        fs::write(
            &argus_file,
            "---\nname: stock-ledger-auditor\ntitle: Argus Winner\nsquad: state\nmodel_tier: deep\ntools: read\n---\nPrompt A\n",
        ).expect("write argus file");

        let swarm_file = swarm_personas.join("stock-ledger-auditor.md");
        fs::write(
            &swarm_file,
            "---\nname: stock-ledger-auditor\ntitle: Swarm Loser\nsquad: state\nmodel_tier: deep\ntools: read\n---\nPrompt B\n",
        ).expect("write swarm file");

        let registry = PersonaRegistry::load(dir.path()).expect("load registry");
        let auditor = registry.get("stock-ledger-auditor").unwrap();
        assert_eq!(auditor.title, "Argus Winner");
    }

    #[test]
    fn test_registry_squad_all_includes_novel_persona() {
        let dir = tempdir().expect("tempdir");
        let personas_dir = dir.path().join(".argus/personas");
        fs::create_dir_all(&personas_dir).expect("create dir");

        let custom_file = personas_dir.join("novel-checker.md");
        fs::write(
            &custom_file,
            "---\nname: novel-checker\ntitle: Novel Checker\nsquad: custom\nmodel_tier: standard\ntools: read\n---\nCheck things.\n",
        ).expect("write file");

        let registry = PersonaRegistry::load(dir.path()).expect("load registry");
        assert_eq!(registry.len(), 13);

        let config = ArgusConfig::default();
        let all_squad = registry.get_squad("all", &config).expect("get squad all");
        let names: Vec<&str> = all_squad.iter().map(|p| p.name.as_str()).collect();
        assert!(names.contains(&"novel-checker"));
        assert_eq!(all_squad.len(), 12); // 11 default workers + 1 novel worker
    }
}
