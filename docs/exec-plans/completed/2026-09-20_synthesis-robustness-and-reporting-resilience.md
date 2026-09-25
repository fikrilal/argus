# Synthesis Robustness and Reporting Resilience

**Plan version:** 1  
**Task ID:** synthesis-robustness-and-reporting-resilience  
**Status:** completed  
**Owner:** ahmad fikril  
**Risk:** low  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** src/synthesis/, src/cli/commands/audit.rs, tests/, Cargo.toml  
**Allowed actions:** edit, verify  
**Maximum risk:** low  
**Repair limit:** 2  
**Task timeout:** 60m  

Date: 2026-09-20  
Origin: Argus dogfooding audit (`.argus/reports/2026-10-01-234141-audit.md` findings #2, #3, #19, #20)

## Objective

Remediate report persistence error handling, string parsing safety, and architecture test alignment:
1. In `src/synthesis/finding.rs`, replace byte-slice indexing with character-boundary-safe slicing in `extract_field_value` so multi-byte Unicode characters with asymmetric case-folding byte lengths (e.g. German `ẞ`) never trigger panic on boundary slicing.
2. In `src/cli/commands/audit.rs`, handle errors from `write_markdown_report` explicitly and print informative warnings to stderr if writing fails, rather than silently discarding errors with `if let Ok(_)`.
3. In `tests/architecture_test.rs`, ensure `test_project_map_drift` performs bidirectional verification between `AGENTS.md` and disk, and ensure all documented modules in `AGENTS.md` exist on disk.

## Constraints

- Zero `unwrap()` or `expect()` in production paths.
- No generic file or module names (`helper`, `manager`, `utils`).
- Must pass `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- All changes must remain uncommitted in the working tree for user review.

## Impact Areas

- CLI commands (`src/cli/`): yes (`audit.rs`)
- Git & Diff engine (`src/git/`): no
- Context & SFD (`src/context/`): no
- Persona registry (`src/persona/`): no
- Subprocess runner (`src/runner/`): no
- Synthesis & UI (`src/synthesis/`): yes
- Config/oracles (`src/config/`, `src/oracles/`): no

## Acceptance Criteria

1. `extract_field_value` never panics on any valid UTF-8 string regardless of casing differences.
2. If saving Markdown report fails in `save_audit_report`, a clear warning is printed to stderr.
3. `test_project_map_drift` passes with bidirectional verification.
4. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Refactor `extract_field_value` in `src/synthesis/finding.rs` to use char boundary slicing.
- [x] Update `save_audit_report` in `src/cli/commands/audit.rs` to handle errors with user diagnostics.
- [x] Update `tests/architecture_test.rs` with bidirectional check.
- [x] Add unit test for Unicode case-folding in `src/synthesis/finding.rs`.
- [x] Run `./scripts/verify.sh`.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
