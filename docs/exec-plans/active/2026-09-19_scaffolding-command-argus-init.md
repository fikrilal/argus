# Scaffolding Command: Argus Init

**Plan version:** 1  
**Task ID:** scaffolding-command-argus-init  
**Status:** active  
**Owner:** ahmad fikril  
**Risk:** low  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** src/cli/commands/, src/main.rs, tests/, Cargo.toml, TODO.md  
**Allowed actions:** edit, verify  
**Maximum risk:** low  
**Repair limit:** 2  
**Task timeout:** 60m  

Date: 2026-09-19  
Related task: TODO.md Phase 1 Task 1.4  

## Objective

Implement the `argus init` scaffolding command in `src/cli/commands/init.rs` to initialize a target repository with a structured `.argus/` directory layout:
- `.argus/config.yaml` with inline documentation and sensible defaults.
- `.argus/oracles.yaml` with starter machine-checked business invariant definitions.
- `.argus/personas/` for repository-specific persona overrides.
- `.argus/context/sfd/` for converted Markdown specifications.
- `.argus/reports/` for audit logs.

Ensure idempotency: warn and refuse to overwrite existing configs unless `--force` is provided.

## Constraints

- Zero `unwrap()` or `expect()` in production paths.
- No generic file or module names (`helper`, `manager`, `utils`).
- Must pass `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- All changes must remain uncommitted in the working tree for user review.

## Impact Areas

- CLI commands (`src/cli/`): yes
- Git & Diff engine (`src/git/`): no
- Persona registry (`src/persona/`): no
- Subprocess runner (`src/runner/`): no
- Synthesis & UI (`src/synthesis/`): no
- Config/oracles (`src/config/`, `src/oracles/`): no

## Acceptance Criteria

1. Running `argus init` in a directory creates:
   - `.argus/config.yaml`
   - `.argus/oracles.yaml`
   - `.argus/personas/`
   - `.argus/context/sfd/`
   - `.argus/reports/`
2. Generated `config.yaml` is valid YAML and deserializable into `ArgusConfig`.
3. Generated `oracles.yaml` is valid YAML.
4. If `.argus/config.yaml` already exists, `argus init` prints a warning and skips overwriting.
5. If `--force` is specified, existing files are safely overwritten.
6. Integration tests in `tests/init_test.rs` verify directory generation, YAML validity, and idempotency.
7. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Create `src/cli/commands/mod.rs` and `src/cli/commands/init.rs`.
- [x] Connect `cli::commands::init::run(&args)` in `src/main.rs`.
- [x] Add integration tests in `tests/init_test.rs`.
- [x] Run `./scripts/verify.sh`.
- [x] Update `TODO.md`.

## Decision Log

- 2026-09-19: Generate starter `config.yaml` and `oracles.yaml` with embedded templates containing helpful inline comments for new adopters.
- 2026-09-19: Implement idempotent skipping: existing files are preserved unless `--force` is supplied.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
Outcome: All 20 tests passed (3 init integration tests, 7 config unit tests, 3 architecture tests, 7 CLI tests), 0 clippy warnings, release build passed.

## Completion Notes

Implemented `argus init` command creating:
- `.argus/config.yaml`
- `.argus/oracles.yaml`
- `.argus/personas/`
- `.argus/context/sfd/`
- `.argus/reports/`
Covered with 3 automated integration tests verifying directory creation, YAML schema validity, idempotency preservation, and `--force` overwrite.
Phase 1 is now 100% completed.
All changes remain uncommitted in the working tree for review.
