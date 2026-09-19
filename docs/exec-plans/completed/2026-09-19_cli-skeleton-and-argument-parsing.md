# CLI Skeleton and Argument Parsing

**Plan version:** 1  
**Task ID:** cli-skeleton-and-argument-parsing  
**Status:** active  
**Owner:** ahmad fikril  
**Risk:** low  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** src/cli/, src/main.rs, tests/, Cargo.toml, TODO.md  
**Allowed actions:** edit, verify  
**Maximum risk:** low  
**Repair limit:** 2  
**Task timeout:** 60m  

Date: 2026-09-19  
Related task: TODO.md Phase 1 Task 1.2  

## Objective

Implement the foundational command-line interface for **Argus** using `clap` (derive API).
Define top-level CLI flags and the four primary subcommands: `audit`, `init`, `personas`, and `resume`.
Wire argument parsing into `src/main.rs` with clean, idiomatic error handling.

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

1. Running `argus --help` displays formatted CLI usage with version, author, and description.
2. The four subcommands are defined with their expected arguments:
   - `audit`: `--squad`, `--sfd`, `--base`, `--staged`, `--concurrency`.
   - `init`: `--force`, `--path`.
   - `personas`: subcommands `list` and `show <name>`.
   - `resume`: `--persona`, `--branch`.
3. Running `argus --version` reports `argus 0.1.0`.
4. Integration tests in `tests/cli_test.rs` verify CLI parsing and exit codes.
5. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Create `src/cli/mod.rs` and `src/cli/args.rs` with `clap` derive structs.
- [x] Wire argument dispatch in `src/main.rs`.
- [x] Add CLI integration tests in `tests/cli_test.rs`.
- [x] Run `./scripts/verify.sh` and fix any format or clippy diagnostics.
- [x] Update `TODO.md` checklist.

## Decision Log

- 2026-09-19: Use `clap` derive mode over builder pattern for compile-time type safety and minimal boilerplate.
- 2026-09-19: Use `-j` shorthand for `--concurrency` in `AuditArgs` to prevent collision with global `-c/--config`.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
Outcome: All 10 tests passed (3 architecture tests, 7 CLI integration tests), 0 clippy warnings, release binary built successfully.

## Completion Notes

Implemented `Cli` top-level args with `--verbose` and `--config`, along with subcommands `audit`, `init`, `personas` (with `list` and `show`), and `resume`.
Verified exit codes and output formatting with automated integration tests in `tests/cli_test.rs`.
All changes remain uncommitted in the working tree for review.

## Risks And Mitigations

- Risk: Generic naming in CLI command handlers.
- Mitigation: Name command modules strictly by their domain action (`audit.rs`, `init.rs`, etc.).
