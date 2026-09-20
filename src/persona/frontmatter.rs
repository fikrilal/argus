use anyhow::{Context, Result};
use gray_matter::Matter;
use gray_matter::engine::YAML;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// The reasoning / intelligence tier required by a persona.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
#[allow(dead_code)]
pub enum ModelTier {
    Fast,
    #[default]
    Standard,
    Deep,
}

/// The origin of a persona definition.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub enum PersonaSource {
    /// Embedded Tier-1 core persona shipped with Argus
    Builtin,
    /// Project-level override or addition from `<repo>/.argus/personas/*.md`
    ProjectOverride(PathBuf),
}

/// A parsed, validated Argus agent persona.
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Persona {
    /// Unique identifier slug (e.g. `stock-ledger-auditor`)
    pub name: String,
    /// Human-readable title
    pub title: String,
    /// Functional squad grouping (e.g. `forms`, `state`, `sync`, `spec`)
    pub squad: String,
    /// Model reasoning tier
    pub model_tier: ModelTier,
    /// Comma-separated or listed tools allowed for this agent
    pub tools: Vec<String>,
    /// The Markdown body containing system prompt instructions
    pub system_prompt: String,
    /// Origin of this persona definition
    pub source: PersonaSource,
}

#[derive(Debug, Deserialize)]
struct RawFrontmatter {
    name: Option<String>,
    title: Option<String>,
    squad: Option<String>,
    model_tier: Option<ModelTier>,
    tools: Option<String>,
}

/// Parses a Markdown document with YAML frontmatter into a validated [`Persona`].
#[allow(dead_code)]
pub fn parse_persona_markdown(raw_markdown: &str, source: PersonaSource) -> Result<Persona> {
    let matter = Matter::<YAML>::new();
    let result = matter.parse(raw_markdown);

    let data = result
        .data
        .with_context(|| "Persona Markdown document is missing YAML frontmatter ('---')")?;

    let raw: RawFrontmatter = data
        .deserialize()
        .with_context(|| "Failed to deserialize persona YAML frontmatter")?;

    let name = raw
        .name
        .filter(|s| !s.trim().is_empty())
        .with_context(|| "Persona frontmatter is missing required 'name' field")?
        .trim()
        .to_string();

    let title = raw
        .title
        .filter(|s| !s.trim().is_empty())
        .with_context(|| "Persona frontmatter is missing required 'title' field")?
        .trim()
        .to_string();

    let squad = raw
        .squad
        .filter(|s| !s.trim().is_empty())
        .with_context(|| "Persona frontmatter is missing required 'squad' field")?
        .trim()
        .to_string();

    let model_tier = raw.model_tier.unwrap_or_default();

    let tools = if let Some(tools_str) = raw.tools {
        tools_str
            .split(',')
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
            .collect()
    } else {
        vec![
            "read".to_string(),
            "grep".to_string(),
            "find".to_string(),
            "ls".to_string(),
            "bash".to_string(),
        ]
    };

    let system_prompt = result.content.trim().to_string();
    if system_prompt.is_empty() {
        anyhow::bail!("Persona Markdown body (system prompt) is empty for persona '{name}'");
    }

    Ok(Persona {
        name,
        title,
        squad,
        model_tier,
        tools,
        system_prompt,
        source,
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;

    #[test]
    fn test_parse_valid_persona_markdown() {
        let raw = r"---
name: stock-ledger-auditor
title: The Stock & Local Ledger Invariant Auditor
squad: state
model_tier: deep
tools: read, grep, find, ls, bash
---

You are an adversarial database and state machine auditor.
";

        let persona = parse_persona_markdown(raw, PersonaSource::Builtin).expect("should parse");
        assert_eq!(persona.name, "stock-ledger-auditor");
        assert_eq!(persona.title, "The Stock & Local Ledger Invariant Auditor");
        assert_eq!(persona.squad, "state");
        assert_eq!(persona.model_tier, ModelTier::Deep);
        assert_eq!(persona.tools, &["read", "grep", "find", "ls", "bash"]);
        assert_eq!(
            persona.system_prompt,
            "You are an adversarial database and state machine auditor."
        );
        assert_eq!(persona.source, PersonaSource::Builtin);
    }

    #[test]
    fn test_parse_persona_default_tools_and_tier() {
        let raw = r"---
name: form-boundary-saboteur
title: Form Boundary Saboteur
squad: forms
---

Attack every form boundary.
";

        let persona = parse_persona_markdown(raw, PersonaSource::Builtin).expect("should parse");
        assert_eq!(persona.model_tier, ModelTier::Standard);
        assert_eq!(persona.tools, &["read", "grep", "find", "ls", "bash"]);
    }

    #[test]
    fn test_parse_persona_missing_name_returns_error() {
        let raw = r"---
title: Form Boundary Saboteur
squad: forms
---

Body here.
";

        let result = parse_persona_markdown(raw, PersonaSource::Builtin);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("missing required 'name'")
        );
    }

    #[test]
    fn test_parse_persona_missing_body_returns_error() {
        let raw = r"---
name: my-persona
title: My Persona
squad: forms
---
";

        let result = parse_persona_markdown(raw, PersonaSource::Builtin);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("system prompt) is empty")
        );
    }
}
