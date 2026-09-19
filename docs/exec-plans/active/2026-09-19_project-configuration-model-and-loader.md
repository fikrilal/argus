# Project Configuration Model and Loader

**Plan version:** 1  
**Task ID:** project-configuration-model-and-loader  
**Status:** active  
**Owner:** ahmad fikril  
**Risk:** low  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** src/config/, src/main.rs, tests/, Cargo.toml, TODO.md  
**Allowed actions:** edit, verify  
**Maximum risk:** low  
**Repair limit:** 2  
**Task timeout:** 60m  

Date: 2026-09-19  
Related task: TODO.md Phase 1 Task 1.3  

## Objective

Implement the project configuration subsystem in `src/config/` for parsing, validating, and providing defaults for `.argus/config.yaml`.
Ensure the loader can discover project-local `.argus/config.yaml` or provide sensible built-in defaults when none exists.

## Constraints

- Zero `unwrap()` or `expect()` in production paths.
- No generic file or module names (`helper`, `manager`, `utils`).
- Must pass `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- All changes must remain uncommitted in the working tree for user review.

## Impact Areas

- CLI commands (`src/cli/`): no
- Git & Diff engine (`src/git/`): no
- Persona registry (`src/persona/`): no
- Subprocess runner (`src/runner/`): no
- Synthesis & UI (`src/synthesis/`): no
- Config/oracles (`src/config/`, `src/oracles/`): yes

## Acceptance Criteria

1. `ArgusConfig` schema models defined with `serde` deserialization:
   - `version: u32`
   - `project: String`
   - `sfd: SfdConfig` (path, active file)
   - `models: ModelTiersConfig` (fast, standard, deep)
   - `squads: HashMap<String, Vec<String>>`
2. `ArgusConfig::default()` provides sensible defaults (version 1, default squads, standard models).
3. `load_config(path: Option<&Path>) -> Result<ArgusConfig>` locates configuration:
   - From explicit CLI flag path if provided.
   - Or by checking `.argus/config.yaml` in current working directory.
   - Falls back to `ArgusConfig::default()` if file does not exist.
   - Returns clear error if file exists but contains invalid YAML.
4. Comprehensive unit tests covering default config, custom YAML parsing, missing file fallback, and invalid YAML errors.
5. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Create `src/config/schema.rs` with serde models and defaults.
- [x] Create `src/config/loader.rs` with path discovery and error handling.
- [x] Create `src/config/mod.rs` re-exporting config types.
- [x] Add unit tests in `src/config/tests.rs`.
- [x] Integrate config loading in `src/main.rs`.
- [x] Run `./scripts/verify.sh` to ensure all checks pass.
- [x] Update `TODO.md`.

## Decision Log

- 2026-09-19: Use `serde_yaml` with `serde(default)` attributes to tolerate omitted fields in user config files.
- 2026-09-19: Fallback discovery checks `.argus/config.yaml`, `.argus/config.yml`, `.swarm/config.yaml`, and `.swarm/config.yml`.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
Outcome: All 17 tests passed (7 config unit tests, 3 architecture tests, 7 CLI integration tests), 0 clippy warnings, release build succeeded.

## Completion Notes

Implemented `ArgusConfig`, `SfdConfig`, `ModelTiersConfig`, and default squads mapping.
Implemented `load_config` supporting explicit paths, auto-discovery in current directory, and clean fallback to defaults.
Covered with 7 unit tests testing default values, custom YAML parsing, discovery, and invalid YAML errors.
All changes remain uncommitted in the working tree for review.
