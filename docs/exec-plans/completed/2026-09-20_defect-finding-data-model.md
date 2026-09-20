# Defect Finding Data Model and Parsing

**Plan version:** 1  
**Task ID:** defect-finding-data-model  
**Status:** active  
**Owner:** ahmad fikril  
**Risk:** low  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** src/synthesis/, src/main.rs, tests/, Cargo.toml, TODO.md  
**Allowed actions:** edit, verify  
**Maximum risk:** low  
**Repair limit:** 2  
**Task timeout:** 60m  

Date: 2026-09-20  
Related task: TODO.md Phase 6 Task 6.1  

## Objective

Implement the structured defect finding data model and output parser in `src/synthesis/finding.rs`:
- Define `Severity` enum (`P0Blocker`, `P1Major`, `P2Polish`) with ordering, formatting, and display helpers.
- Define `Finding` struct holding structured defect attributes: severity, reporting persona, title, file path, line number, description, failure scenario, and recommended fix.
- Implement `parse_agent_findings(persona_name: &str, raw_output: &str) -> Vec<Finding>` extracting structured findings from agent markdown output.
- Enforce that `src/synthesis/` adheres to all architectural boundaries: zero `println!` calls, no imports from `cli` or `runner`.

## Constraints

- Zero `unwrap()` or `expect()` in production paths.
- No generic file or module names (`helper`, `manager`, `utils`).
- Must pass `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- All changes must remain uncommitted in the working tree for user review.

## Impact Areas

- CLI commands (`src/cli/`): no
- Git & Diff engine (`src/git/`): no
- Context & SFD (`src/context/`): no
- Persona registry (`src/persona/`): no
- Subprocess runner (`src/runner/`): no
- Synthesis & UI (`src/synthesis/`): yes
- Config/oracles (`src/config/`, `src/oracles/`): no

## Acceptance Criteria

1. `Severity` enum supports:
   - `P0Blocker`: data loss, stock reversibility, deadlock, crash on 200 OK nulls.
   - `P1Major`: bypassed form validations, cascade dropdown errors, missing business rules.
   - `P2Polish`: minor formatting, soft limits, suggestions.
   - Methods: `as_str()`, `badge()`.
2. `Finding` struct supports serialization/deserialization with `serde`.
3. `parse_agent_findings` correctly parses agent outputs containing `[VIOLATION]` blocks, extracting targets, issues, scenarios, and code fixes.
4. If output contains `[STATUS: PASS]`, yields an empty vector.
5. Unit tests cover parsing single violations, multiple violations, pass status, and target parsing (`file:line`).
6. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Create `src/synthesis/mod.rs` and `src/synthesis/finding.rs`.
- [x] Implement `Severity` and `Finding`.
- [x] Implement `parse_agent_findings`.
- [x] Add unit tests in `src/synthesis/finding.rs`.
- [x] Wire `mod synthesis;` in `src/main.rs`.
- [x] Run `./scripts/verify.sh`.
- [x] Update `TODO.md`.

## Decision Log

- 2026-09-20: Auto-derive severity from persona type and issue keywords (e.g. stock/ledger/deadlock -> P0; validation/forms -> P1; suggestions -> P2) if not explicitly tagged by the agent.
- 2026-09-20: Pass `Severity` by value in methods (`code`, `label`, `badge`) to satisfy Clippy's `trivially-copy-pass-by-ref` lint.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
Outcome: All 64 tests passed (5 finding unit tests, 39 unittests, 6 mechanical architecture tests, 11 CLI tests, 3 init tests), 0 clippy warnings (including pedantic checks), release build passed.

## Completion Notes

Implemented `Severity` (P0/P1/P2) and `Finding` data structures with `serde` support.
Implemented `parse_agent_findings` extracting structured findings from agent output.
Wired `mod synthesis;` without violating layer boundary or println quarantine rules.
All changes remain uncommitted in the working tree for review.
