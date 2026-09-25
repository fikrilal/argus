# Runner Subprocess and Concurrency Safety

**Plan version:** 1  
**Task ID:** runner-subprocess-and-concurrency-safety  
**Status:** completed  
**Owner:** ahmad fikril  
**Risk:** medium  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** src/runner/, src/cli/commands/resume.rs, tests/, Cargo.toml  
**Allowed actions:** edit, verify  
**Maximum risk:** medium  
**Repair limit:** 2  
**Task timeout:** 60m  

Date: 2026-09-20  
Origin: Argus dogfooding audit (`.argus/reports/2026-10-01-234141-audit.md` findings #6, #13, #14, #15, #16, #17, #18)

## Objective

Remediate concurrency and subprocess safety defects discovered by Argus in `src/runner/` and `src/cli/commands/resume.rs`:
1. Add `cmd.kill_on_drop(true)` in `src/runner/launcher.rs` to guarantee child `pi` processes are terminated if a task times out or is dropped.
2. Add error context on temporary prompt file creation, writing, and flushing.
3. Replace `Vec<JoinHandle>` with `tokio::task::JoinSet` in `src/runner/pool.rs` to propagate task aborts if `execute_all` is cancelled or dropped.
4. Replace blocking synchronous stdin read (`std::io::stdin().lock().read_line`) in `src/cli/commands/resume.rs` with asynchronous `tokio::io::AsyncBufReadExt` on `tokio::io::stdin()`.
5. Fix prompt range formatting in `resume.rs` to dynamically display `[1-{} or name]` based on `registry.all().len()`.

## Constraints

- Zero `unwrap()` or `expect()` in production paths.
- No generic file or module names (`helper`, `manager`, `utils`).
- Strict non-blocking async operations (`std::process::Command` is forbidden; blocking stdin on async thread is forbidden).
- Must pass `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- All changes must remain uncommitted in the working tree for user review.

## Impact Areas

- CLI commands (`src/cli/`): yes (`resume.rs`)
- Git & Diff engine (`src/git/`): no
- Context & SFD (`src/context/`): no
- Persona registry (`src/persona/`): no
- Subprocess runner (`src/runner/`): yes
- Synthesis & UI (`src/synthesis/`): no
- Config/oracles (`src/config/`, `src/oracles/`): no

## Acceptance Criteria

1. Child processes spawned by `launch_agent` configure `.kill_on_drop(true)`.
2. Temporary prompt file flushing has informative error context with path display.
3. `SwarmPool::execute_all` uses `tokio::task::JoinSet` to prevent detached background tasks on cancellation.
4. `resume::run` uses `tokio::io::stdin()` with `AsyncBufReadExt` so Tokio worker threads are never blocked by user input.
5. `resume::run` displays dynamic bounds `[1-{} or name]`.
6. Unit and integration tests pass without warnings or regressions.
7. Verification gate `./scripts/verify.sh` passes cleanly.

## Implementation Checklist

- [x] Add `cmd.kill_on_drop(true)` and context in `src/runner/launcher.rs`.
- [x] Migrate `SwarmPool::execute_all` to `tokio::task::JoinSet` in `src/runner/pool.rs`.
- [x] Migrate `resume::run` to `tokio::io::stdin()` and dynamic count formatting in `src/cli/commands/resume.rs`.
- [x] Verify with `cargo test` and `./scripts/verify.sh`.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
