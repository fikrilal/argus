# Swarm Concurrency Pool and Progress UI

**Plan version:** 1  
**Task ID:** swarm-concurrency-pool-and-progress-ui  
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
Related tasks: TODO.md Phase 5 Task 5.3 & Task 5.4  

## Objective

Implement the bounded concurrent execution engine and live terminal feedback UI in `src/runner/`:
1. `src/runner/pool.rs`: Implement `SwarmPool` managing parallel execution of agent plans using `tokio::sync::Semaphore` bounded to `concurrency_limit` (default: 4). Wrap agent runs in per-agent timeouts (default: 180s) and handle partial failures gracefully.
2. `src/runner/progress.rs`: Implement multi-spinner progress management using `indicatif::MultiProgress`, showing concurrent spinner lines for each active agent with clean fallback for non-TTY / CI environments.
3. Wire the swarm execution into `src/main.rs` for `argus audit`.

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

1. `SwarmPool` struct encapsulates:
   - `concurrency_limit: usize`
   - `timeout_duration: Duration`
   - `pi_bin_override: Option<String>`
2. `execute_all(plans: Vec<AgentExecutionPlan>, working_dir: &Path) -> Result<Vec<AgentRunResult>>`:
   - Enforces that no more than `concurrency_limit` processes run simultaneously.
   - If an agent task exceeds timeout, records an error result instead of hanging.
   - If one agent fails or crashes, remaining agents complete without interruption.
   - Displays real-time progress using `indicatif::MultiProgress`.
3. Unit tests verify bounded concurrency throttling, timeout handling, and mock agent executions.
4. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Create `src/runner/progress.rs` for live progress spinners.
- [x] Create `src/runner/pool.rs` with `tokio::sync::Semaphore`.
- [x] Export `SwarmPool` in `src/runner/mod.rs`.
- [x] Add unit tests in `src/runner/pool.rs`.
- [x] Integrate `SwarmPool` execution in `src/main.rs`.
- [x] Run `./scripts/verify.sh`.
- [x] Update `TODO.md`.

## Decision Log

- 2026-09-20: Use `tokio::sync::Semaphore` with `acquire_owned()` across spawned Tokio tasks for reliable permit release on task completion or timeout.
- 2026-09-20: Refactored audit command execution from `main.rs` into `src/cli/commands/audit.rs` to satisfy Clippy's `too-many-lines` lint.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
Outcome: All 59 tests passed (39 unittests including pool concurrency and timeout tests, 6 mechanical architecture tests, 11 CLI tests, 3 init tests), 0 clippy warnings (including pedantic checks), release build passed.

## Completion Notes

Implemented `SwarmPool` managing bounded concurrency execution via `tokio::sync::Semaphore`.
Implemented `SwarmProgressTracker` displaying concurrent, live progress spinners for each active agent via `indicatif`.
Integrated `SwarmPool` into the `argus audit` command.
Phase 5 is now 100% completed.
All changes remain uncommitted in the working tree for review.
