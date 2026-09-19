# <Plan Title>

**Plan version:** 1  
**Task ID:** lowercase-kebab-case-task-id  
**Status:** active  
**Owner:** <name>  
**Risk:** low | medium | high  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** src/, tests/, Cargo.toml, docs/  
**Allowed actions:** edit, verify  
**Maximum risk:** low | medium | high  
**Repair limit:** 2  
**Task timeout:** 90m  

Date: YYYY-MM-DD  
Related issue/PR: <link or N/A>  

## Objective

Describe the concrete outcome this task must deliver.

## Constraints

- Architecture constraints:
- Rust runtime/safety constraints: Zero `unwrap()`, zero `unsafe`, zero compiler/clippy warnings.
- Out of scope:

## Impact Areas

- CLI commands (`src/cli/`): yes | no
- Git & Diff engine (`src/git/`): yes | no
- Persona registry (`src/persona/`): yes | no
- Subprocess runner (`src/runner/`): yes | no
- Synthesis & UI (`src/synthesis/`): yes | no
- Config/oracles (`src/config/`, `src/oracles/`): yes | no

## Acceptance Criteria

1. 
2. 
3. 

## Implementation Checklist

- [ ] Step 1
- [ ] Step 2
- [ ] Step 3

## Decision Log

- YYYY-MM-DD: <decision> -> <reason>

## Verification

List exact commands and outcomes.

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

## Risks And Mitigations

- Risk:
- Mitigation:

## Completion Notes

Summarize what shipped, what changed, and any important caveats.
