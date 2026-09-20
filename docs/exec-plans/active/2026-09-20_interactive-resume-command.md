# Interactive Resume Command: Argus Resume

**Plan version:** 1  
**Task ID:** interactive-resume-command  
**Status:** active  
**Owner:** ahmad fikril  
**Risk:** low  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** src/cli/commands/, src/main.rs, tests/, Cargo.toml, TODO.md  
**Allowed actions:** edit, verify  
**Maximum risk:** low  
**Repair limit:** 2  
**Task timeout:** 60m  

Date: 2026-09-20  
Related task: TODO.md Phase 7 Task 7.1  

## Objective

Implement `src/cli/commands/resume.rs` powering `argus resume`:
- Detect the active Git branch slug (`git::detect_current_branch` + `git::slugify_branch_name`).
- If `--persona <name>` is provided, target `argus/<branch-slug>/<persona-name>`.
- If `--branch <branch>` is provided, use the custom branch slug.
- If no persona is passed, list available personas from `PersonaRegistry` for interactive selection.
- Hand over the terminal to `pi --resume <session_name>` with inherited stdio (`tokio::process::Command` with `.status().await`).
- Fail gracefully if `pi` binary is not found on PATH or session name is unknown.

## Constraints

- Zero `unwrap()` or `expect()` in production paths.
- No generic file or module names (`helper`, `manager`, `utils`).
- Must strictly use `tokio::process::Command` (no blocking `std::process::Command`). Enforced by `tests/architecture_test.rs`.
- Must pass `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- All changes must remain uncommitted in the working tree for user review.

## Impact Areas

- CLI commands (`src/cli/`): yes
- Git & Diff engine (`src/git/`): no
- Context & SFD (`src/context/`): no
- Persona registry (`src/persona/`): no
- Subprocess runner (`src/runner/`): no
- Synthesis & UI (`src/synthesis/`): no
- Config/oracles (`src/config/`, `src/oracles/`): no

## Acceptance Criteria

1. Function `resolve_resume_session_name(args: &ResumeArgs, current_branch_slug: &str, registry: &PersonaRegistry) -> Result<String>`:
   - Returns explicit session name if `--persona` is passed.
   - Allows `--branch` override.
   - Validates that the requested persona exists in the registry.
2. Function `run(args: &ResumeArgs, registry: &PersonaRegistry, current_dir: &Path) -> Result<()>`:
   - Resolves target session name.
   - Launches interactive `pi --resume <session_name>` inheriting terminal stdio.
3. Unit tests in `src/cli/commands/resume.rs` verify session name resolution with and without branch overrides, and invalid persona error reporting.
4. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Create `src/cli/commands/resume.rs`.
- [x] Export `resume` in `src/cli/commands/mod.rs`.
- [x] Wire `argus resume` in `src/main.rs`.
- [x] Add unit tests in `src/cli/commands/resume.rs`.
- [x] Run `./scripts/verify.sh`.
- [x] Update `TODO.md`.

## Decision Log

- 2026-09-20: Separate session name resolution into a pure testable function `resolve_resume_session_name` and the async process launcher `run`.
- 2026-09-20: Use `tokio::process::Command` to adhere to our mechanical async execution guardrail.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
Outcome: All 76 tests passed (4 resume unit tests, 12 CLI tests, 51 unittests, 6 mechanical architecture tests, 3 init tests), 0 clippy warnings (including pedantic checks), release build passed.

## Completion Notes

Implemented `argus resume` supporting direct `--persona <name>`, `--branch <override>`, and interactive numbered selection.
Hands over terminal to `pi --resume <session_name>`.
All changes remain uncommitted in the working tree for review.
