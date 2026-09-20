# End-to-End Audit Pipeline Integration Tests

**Plan version:** 1  
**Task ID:** end-to-end-audit-pipeline-test  
**Status:** active  
**Owner:** ahmad fikril  
**Risk:** low  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** tests/, src/cli/commands/audit.rs, src/main.rs, Cargo.toml, TODO.md  
**Allowed actions:** edit, verify  
**Maximum risk:** low  
**Repair limit:** 2  
**Task timeout:** 60m  

Date: 2026-09-20  
Related task: TODO.md Phase 7 Task 7.2  

## Objective

Implement end-to-end integration tests in `tests/audit_test.rs` to verify the entire Argus audit lifecycle:
- Test successful audit run where all personas pass, verifying exit code `0`, terminal summary, and persistent Markdown report generation.
- Test audit run with defects (P0/P1), verifying exit code `1`, terminal summary highlighting blockers/majors, and Markdown report output.
- Test squad targeting (`--squad forms`).
- Test specification document passing (`--sfd <path>`).
- Use mock `pi` executable via `ARGUS_PI_BIN` to ensure fast, deterministic, and hermetic execution without external network or LLM dependencies.

## Constraints

- Zero `unwrap()` or `expect()` in production paths.
- No generic file or module names (`helper`, `manager`, `utils`).
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

1. `tests/audit_test.rs` executes real `argus audit` CLI invocations against isolated temporary directories.
2. Verified scenarios:
   - Clean audit scenario: returns exit code `0` and prints `ARGUS AUDIT PASSED`.
   - Violation scenario: returns non-zero exit code (`1`) and prints `ARGUS AUDIT ACTION REQUIRED`.
   - Report verification: asserts `.argus/reports/*-audit.md` is generated on disk and contains expected findings.
3. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Create `tests/audit_test.rs` with hermetic mock script runner.
- [x] Implement clean pass test.
- [x] Implement defect detection test.
- [x] Implement squad and SFD flag tests.
- [x] Run `./scripts/verify.sh`.
- [x] Update `TODO.md`.

## Decision Log

- 2026-09-20: Configure mock `pi` behavior via environment variables (`ARGUS_PI_BIN`) in test processes to ensure hermetic and zero-network execution.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
Outcome: All 79 tests passed (3 e2e audit tests, 12 CLI tests, 6 mechanical architecture tests, 3 init tests, 55 unittests), 0 clippy warnings (including pedantic checks), release build passed.

## Completion Notes

Implemented comprehensive end-to-end integration tests in `tests/audit_test.rs`.
Verified clean audits, defect detection, exit code 1 vs 0 contracts, terminal dashboard output, and `.argus/reports/` markdown generation.
All changes remain uncommitted in the working tree for review.
