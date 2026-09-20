# Embedded Tier-1 Base Personas

**Plan version:** 1  
**Task ID:** embedded-tier-1-base-personas  
**Status:** active  
**Owner:** ahmad fikril  
**Risk:** low  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** src/persona/, tests/, Cargo.toml, TODO.md  
**Allowed actions:** edit, verify  
**Maximum risk:** low  
**Repair limit:** 2  
**Task timeout:** 60m  

Date: 2026-09-19  
Related task: TODO.md Phase 4 Task 4.2  

## Objective

Embed the 12 core Tier-1 adversarial agent personas into the compiled Argus binary using `include_str!`.
Implement `src/persona/builtin.rs` providing a catalog function `get_builtin_personas() -> Result<Vec<Persona>>`.
Ensure every embedded persona Markdown file has valid YAML frontmatter, strict squad grouping, and rigorous adversarial system prompts.

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

1. 12 persona Markdown files created under `src/persona/builtin/`:
   - `sfd-clause-detective.md` (Squad: `spec`, Tier: `deep`)
   - `id-regulatory-sentinel.md` (Squad: `forms`, Tier: `standard`)
   - `rbac-identity-gatekeeper.md` (Squad: `spec`, Tier: `standard`)
   - `form-boundary-saboteur.md` (Squad: `forms`, Tier: `standard`)
   - `cascade-dropdown-glitcher.md` (Squad: `forms`, Tier: `standard`)
   - `concurrency-double-tapper.md` (Squad: `forms`, Tier: `standard`)
   - `stock-ledger-auditor.md` (Squad: `state`, Tier: `deep`)
   - `orphan-cascade-hunter.md` (Squad: `state`, Tier: `standard`)
   - `payload-pessimist.md` (Squad: `sync`, Tier: `standard`)
   - `sync-deadlock-guard.md` (Squad: `sync`, Tier: `standard`)
   - `hardware-sensor-adversary.md` (Squad: `sync`, Tier: `standard`)
   - `lead-qa-synthesizer.md` (Squad: `synthesis`, Tier: `deep`)
2. `get_builtin_personas() -> Result<Vec<Persona>>` parses and returns all 12 personas with `PersonaSource::Builtin`.
3. Unit tests verify all 12 personas parse successfully without errors and unique names.
4. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Create `src/persona/builtin/` directory with 12 Markdown persona files.
- [x] Implement `src/persona/builtin.rs` using `include_str!`.
- [x] Export `get_builtin_personas` in `src/persona/mod.rs`.
- [x] Add unit tests verifying parsing of all 12 personas.
- [x] Run `./scripts/verify.sh`.
- [x] Update `TODO.md`.

## Decision Log

- 2026-09-19: Embed personas via `include_str!` so the compiled `argus` binary is 100% self-contained without needing external prompt assets installed on disk.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
Outcome: All 40 tests passed (12 builtin persona tests + 1 test asserting all 12 parse cleanly with unique names, 4 frontmatter tests, 4 bundle tests, 6 SFD tests, 5 branch tests, 7 config tests, 3 architecture tests, 7 CLI tests, 3 init tests), 0 clippy warnings, release build passed.

## Completion Notes

All 12 Tier-1 core agent personas have been written and embedded into the binary:
1. `sfd-clause-detective` (Squad: spec, Tier: deep)
2. `id-regulatory-sentinel` (Squad: forms, Tier: standard)
3. `rbac-identity-gatekeeper` (Squad: spec, Tier: standard)
4. `form-boundary-saboteur` (Squad: forms, Tier: standard)
5. `cascade-dropdown-glitcher` (Squad: forms, Tier: standard)
6. `concurrency-double-tapper` (Squad: forms, Tier: standard)
7. `stock-ledger-auditor` (Squad: state, Tier: deep)
8. `orphan-cascade-hunter` (Squad: state, Tier: standard)
9. `payload-pessimist` (Squad: sync, Tier: standard)
10. `sync-deadlock-guard` (Squad: sync, Tier: standard)
11. `hardware-sensor-adversary` (Squad: sync, Tier: standard)
12. `lead-qa-synthesizer` (Squad: synthesis, Tier: deep)
All changes remain uncommitted in the working tree for review.
