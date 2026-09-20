# Two-Tier Persona Registry

**Plan version:** 1  
**Task ID:** two-tier-persona-registry  
**Status:** active  
**Owner:** ahmad fikril  
**Risk:** low  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** src/persona/, src/main.rs, tests/, Cargo.toml, TODO.md  
**Allowed actions:** edit, verify  
**Maximum risk:** low  
**Repair limit:** 2  
**Task timeout:** 60m  

Date: 2026-09-19  
Related task: TODO.md Phase 4 Task 4.3  

## Objective

Implement `PersonaRegistry` in `src/persona/registry.rs` providing two-tier persona resolution:
1. Seed the registry with all 12 compiled-in Tier-1 base personas.
2. Scan `<project_root>/.argus/personas/*.md` (and fallback `.swarm/personas/*.md`) for project-level customizations.
3. Override base personas when names match; register novel personas dynamically.
4. Support squad resolution (`forms`, `state`, `sync`, `spec`, `all`) mapped via `ArgusConfig.squads`.

## Constraints

- Zero `unwrap()` or `expect()` in production paths.
- No generic file or module names (`helper`, `manager`, `utils`).
- Must pass `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- All changes must remain uncommitted in the working tree for user review.

## Impact Areas

- CLI commands (`src/cli/`): no
- Git & Diff engine (`src/git/`): no
- Context & SFD (`src/context/`): no
- Persona registry (`src/persona/`): yes
- Subprocess runner (`src/runner/`): no
- Synthesis & UI (`src/synthesis/`): no
- Config/oracles (`src/config/`, `src/oracles/`): no

## Acceptance Criteria

1. `PersonaRegistry` loads all 12 Tier-1 builtins by default.
2. If `.argus/personas/<name>.md` exists:
   - If `<name>` matches a built-in persona, it overrides the built-in and sets `source = ProjectOverride(path)`.
   - If `<name>` is novel, it is dynamically registered.
3. `registry.get(name: &str) -> Option<&Persona>` retrieves a persona by name.
4. `registry.get_squad(squad_name: &str, config: &ArgusConfig) -> Result<Vec<&Persona>>` returns personas for the requested squad, failing informatively if a requested persona does not exist.
5. `registry.all() -> Vec<&Persona>` returns all personas sorted by name.
6. Comprehensive unit tests covering default loading, overrides, novel registration, and squad queries.
7. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Create `src/persona/registry.rs`.
- [x] Export `PersonaRegistry` in `src/persona/mod.rs`.
- [x] Wire registry into `src/main.rs` in `audit` and `personas list` handlers.
- [x] Add unit tests in `src/persona/registry.rs`.
- [x] Run `./scripts/verify.sh`.
- [x] Update `TODO.md`.

## Decision Log

- 2026-09-19: Support both `.argus/personas/` and `.swarm/personas/` for seamless compatibility.
- 2026-09-19: Move CLI personas list/show handler to `src/cli/commands/personas.rs` and verify with automated CLI tests.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
Outcome: All 46 tests passed (5 registry unit tests, 12 builtin tests, 4 frontmatter tests, 4 bundle tests, 6 SFD tests, 5 branch tests, 7 config tests, 3 architecture tests, 11 CLI tests, 3 init tests), 0 clippy warnings, release build passed.

## Completion Notes

Implemented `PersonaRegistry` with two-tier resolution (built-ins + project overrides/additions).
Implemented squad resolution and CLI commands `argus personas list` and `argus personas show <name>`.
Covered with unit and integration tests.
Phase 4 is now 100% completed.
All changes remain uncommitted in the working tree for review.
