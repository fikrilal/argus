use anyhow::Result;

use crate::persona::frontmatter::{Persona, PersonaSource, parse_persona_markdown};

const PERSONA_SFD_CLAUSE_DETECTIVE: &str = include_str!("builtin/sfd-clause-detective.md");
const PERSONA_ID_REGULATORY_SENTINEL: &str = include_str!("builtin/id-regulatory-sentinel.md");
const PERSONA_RBAC_IDENTITY_GATEKEEPER: &str = include_str!("builtin/rbac-identity-gatekeeper.md");
const PERSONA_FORM_BOUNDARY_SABOTEUR: &str = include_str!("builtin/form-boundary-saboteur.md");
const PERSONA_CASCADE_DROPDOWN_GLITCHER: &str =
    include_str!("builtin/cascade-dropdown-glitcher.md");
const PERSONA_CONCURRENCY_DOUBLE_TAPPER: &str =
    include_str!("builtin/concurrency-double-tapper.md");
const PERSONA_STOCK_LEDGER_AUDITOR: &str = include_str!("builtin/stock-ledger-auditor.md");
const PERSONA_ORPHAN_CASCADE_HUNTER: &str = include_str!("builtin/orphan-cascade-hunter.md");
const PERSONA_PAYLOAD_PESSIMIST: &str = include_str!("builtin/payload-pessimist.md");
const PERSONA_SYNC_DEADLOCK_GUARD: &str = include_str!("builtin/sync-deadlock-guard.md");
const PERSONA_HARDWARE_SENSOR_ADVERSARY: &str =
    include_str!("builtin/hardware-sensor-adversary.md");
const PERSONA_LEAD_QA_SYNTHESIZER: &str = include_str!("builtin/lead-qa-synthesizer.md");

const ALL_BUILTIN_PERSONAS_RAW: &[&str] = &[
    PERSONA_SFD_CLAUSE_DETECTIVE,
    PERSONA_ID_REGULATORY_SENTINEL,
    PERSONA_RBAC_IDENTITY_GATEKEEPER,
    PERSONA_FORM_BOUNDARY_SABOTEUR,
    PERSONA_CASCADE_DROPDOWN_GLITCHER,
    PERSONA_CONCURRENCY_DOUBLE_TAPPER,
    PERSONA_STOCK_LEDGER_AUDITOR,
    PERSONA_ORPHAN_CASCADE_HUNTER,
    PERSONA_PAYLOAD_PESSIMIST,
    PERSONA_SYNC_DEADLOCK_GUARD,
    PERSONA_HARDWARE_SENSOR_ADVERSARY,
    PERSONA_LEAD_QA_SYNTHESIZER,
];

/// Returns all 12 embedded Tier-1 base personas compiled into the binary.
#[allow(dead_code)]
pub fn get_builtin_personas() -> Result<Vec<Persona>> {
    let mut personas = Vec::with_capacity(ALL_BUILTIN_PERSONAS_RAW.len());
    for raw in ALL_BUILTIN_PERSONAS_RAW {
        let persona = parse_persona_markdown(raw, PersonaSource::Builtin)?;
        personas.push(persona);
    }
    Ok(personas)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_all_12_builtin_personas_parse_successfully() {
        let personas = get_builtin_personas().expect("all builtin personas must parse");
        assert_eq!(personas.len(), 12);

        let mut names = HashSet::new();
        for persona in &personas {
            assert!(
                !persona.name.trim().is_empty(),
                "persona name must not be empty"
            );
            assert!(
                !persona.title.trim().is_empty(),
                "persona title must not be empty for {}",
                persona.name
            );
            assert!(
                !persona.system_prompt.trim().is_empty(),
                "system prompt must not be empty for {}",
                persona.name
            );
            assert_eq!(persona.source, PersonaSource::Builtin);

            assert!(
                names.insert(persona.name.clone()),
                "duplicate persona name detected: {}",
                persona.name
            );
        }

        // Verify specific key personas are present
        assert!(names.contains("sfd-clause-detective"));
        assert!(names.contains("id-regulatory-sentinel"));
        assert!(names.contains("stock-ledger-auditor"));
        assert!(names.contains("form-boundary-saboteur"));
        assert!(names.contains("payload-pessimist"));
        assert!(names.contains("lead-qa-synthesizer"));
    }
}
