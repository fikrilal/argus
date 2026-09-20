# Pi Session Naming and Subprocess Launcher

**Plan version:** 1  
**Task ID:** pi-session-naming-and-subprocess-launcher  
**Status:** active  
**Owner:** ahmad fikril  
**Risk:** medium  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** src/runner/, src/main.rs, tests/, Cargo.toml, TODO.md  
**Allowed actions:** edit, verify  
**Maximum risk:** medium  
**Repair limit:** 2  
**Task timeout:** 60m  

Date: 2026-09-20  
Related tasks: TODO.md Phase 5 Task 5.1 & Task 5.2  

## Objective

Implement the foundational execution primitives in `src/runner/`:
1. `src/runner/session.rs`: Deterministic session naming engine constructing `argus/<branch-slug>/<persona-slug>` identifiers.
2. `src/runner/launcher.rs`: Asynchronous process launcher wrapping `pi` CLI in non-interactive mode (`pi -p`), capturing stdout/stderr, measuring wall-clock duration, and handling missing binary errors cleanly.
3. Support environment override (`ARGUS_PI_BIN`) to enable deterministic mock execution in automated tests without triggering live LLM inference.

## Constraints

- Zero `unwrap()` or `expect()` in production paths.
- No generic file or module names (`helper`, `manager`, `utils`).
- Must strictly use `tokio::process::Command` (no blocking `std::process::Command`). Enforced by `tests/architecture_test.rs`.
- Must pass `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- All changes must remain uncommitted in the working tree for user review.

## Impact Areas

- CLI commands (`src/cli/`): no
- Git & Diff engine (`src/git/`): no
- Context & SFD (`src/context/`): no
- Persona registry (`src/persona/`): no
- Subprocess runner (`src/runner/`): yes
- Synthesis & UI (`src/synthesis/`): no
- Config/oracles (`src/config/`, `src/oracles/`): no

## Acceptance Criteria

1. `format_session_name(branch_slug: &str, persona_slug: &str) -> String` produces `argus/<branch_slug>/<persona_slug>`.
2. `AgentRunResult` holds:
   - `persona_name: String`
   - `session_name: String`
   - `exit_code: i32`
   - `stdout: String`
   - `stderr: String`
   - `duration: Duration`
3. `launch_agent(plan: &AgentExecutionPlan, pi_bin_override: Option<&str>) -> Result<AgentRunResult>`:
   - Launches `pi -p --name <session_name> --append-system-prompt <prompt_file> "<payload>"`.
   - Captures output asynchronously without blocking the Tokio runtime.
   - Accurately captures exit code and duration.
   - Fails informatively if `pi` binary is not found on `PATH`.
4. Unit tests cover session naming, successful execution via mock shell script, non-zero exit code capture, and missing binary error handling.
5. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Create `src/runner/mod.rs`, `src/runner/session.rs`, and `src/runner/launcher.rs`.
- [x] Implement `format_session_name`.
- [x] Implement `AgentExecutionPlan` and `AgentRunResult`.
- [x] Implement `launch_agent` using `tokio::process::Command`.
- [x] Add unit tests in `src/runner/launcher.rs` and `src/runner/session.rs`.
- [x] Wire `mod runner;` into `src/main.rs`.
- [x] Run `./scripts/verify.sh`.
- [x] Update `TODO.md`.

## Decision Log

- 2026-09-20: Pass prompt instructions to `pi` by writing the persona system prompt to a temporary file passed via `--append-system-prompt`, preventing command-line length limits.
- 2026-09-20: Support `ARGUS_PI_BIN` env var so tests can run mock binaries without external dependencies.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
Outcome: All 50 tests passed (2 launcher unit tests with mock shell binary and missing binary handling, 2 session name tests, 5 registry tests, 12 builtin tests, 4 frontmatter tests, 4 bundle tests, 6 SFD tests, 5 branch tests, 7 config tests, 3 architecture tests, 11 CLI tests, 3 init tests), 0 clippy warnings, release build passed.

## Completion Notes

Implemented `format_session_name` for deterministic session identification.
Implemented `launch_agent` using `tokio::process::Command` with temporary file prompt passing and error diagnostics.
All changes remain uncommitted in the working tree for review.
