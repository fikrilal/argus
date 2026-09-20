# Terminal Dashboard Renderer

**Plan version:** 1  
**Task ID:** terminal-dashboard-renderer  
**Status:** active  
**Owner:** ahmad fikril  
**Risk:** low  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** src/synthesis/, src/cli/commands/audit.rs, src/main.rs, tests/, Cargo.toml, TODO.md  
**Allowed actions:** edit, verify  
**Maximum risk:** low  
**Repair limit:** 2  
**Task timeout:** 60m  

Date: 2026-09-20  
Related task: TODO.md Phase 6 Task 6.3  

## Objective

Implement the terminal dashboard renderer in `src/synthesis/terminal.rs`:
- Render colorized terminal summaries of `AuditSynthesis` results.
- Render prominent status banner: green `✔ AUDIT PASSED` vs red `✖ AUDIT ACTION REQUIRED`.
- Render structured finding cards displaying:
  - Colorized severity badge (`[P0 - BLOCKER]`, `[P1 - MAJOR]`, `[P2 - POLISH]`).
  - Target file and line number.
  - Persona attribution.
  - Failure scenario and concrete recommended code fixes.
- List personas that reported clean passes.
- Ensure strict adherence to our mechanical architecture rules: `src/synthesis/terminal.rs` produces a formatted `String` without calling unquarantined `println!` macros.
- Wire dashboard output into `src/cli/commands/audit.rs`.

## Constraints

- Zero `unwrap()` or `expect()` in production paths.
- No generic file or module names (`helper`, `manager`, `utils`).
- Must pass `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- All changes must remain uncommitted in the working tree for user review.

## Impact Areas

- CLI commands (`src/cli/`): yes (rendering dashboard in `audit.rs`)
- Git & Diff engine (`src/git/`): no
- Context & SFD (`src/context/`): no
- Persona registry (`src/persona/`): no
- Subprocess runner (`src/runner/`): no
- Synthesis & UI (`src/synthesis/`): yes
- Config/oracles (`src/config/`, `src/oracles/`): no

## Acceptance Criteria

1. `render_terminal_dashboard(synthesis: &AuditSynthesis) -> String` returns colorized terminal summary.
2. If `synthesis.is_passed()`, renders a green passing banner and lists passing personas.
3. If defects exist, renders an action-required banner and iterates through findings, formatting target, issue, scenario, and code fixes.
4. Unit tests in `src/synthesis/terminal.rs` verify clean pass output and finding formatting.
5. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Create `src/synthesis/terminal.rs`.
- [x] Export `render_terminal_dashboard` in `src/synthesis/mod.rs`.
- [x] Add unit tests in `src/synthesis/terminal.rs`.
- [x] Call `render_terminal_dashboard` in `src/cli/commands/audit.rs`.
- [x] Run `./scripts/verify.sh`.
- [x] Update `TODO.md`.

## Decision Log

- 2026-09-20: Render dashboard as a `String` from `synthesis::terminal` to satisfy our architecture lint preventing unquarantined `println!` in engine crates.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
Outcome: All 68 tests passed (2 terminal unit tests, 2 synthesis tests, 5 finding tests, 39 unittests, 6 mechanical architecture tests, 11 CLI tests, 3 init tests), 0 clippy warnings (including pedantic checks), release build passed.

## Completion Notes

Implemented `render_terminal_dashboard` generating clean, colorized terminal cards with severity badges, target paths, scenarios, code fixes, and passing personas.
Wired dashboard into `src/cli/commands/audit.rs`.
All changes remain uncommitted in the working tree for review.
