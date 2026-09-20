# Markdown Report Generator

**Plan version:** 1  
**Task ID:** markdown-report-generator  
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
Related task: TODO.md Phase 6 Task 6.4  

## Objective

Implement the persistent Markdown report generator in `src/synthesis/markdown.rs`:
- Generate formatted, standalone Markdown reports capturing full audit metadata (timestamp, branch, squad, active SFD, verdict, findings table, code fixes, and passing personas).
- Save reports under `<project_root>/.argus/reports/YYYY-MM-DD-HHMMSS-audit.md` (or `.swarm/reports/`).
- Wire report generation into `src/cli/commands/audit.rs`.
- Ensure strict adherence to mechanical architecture rules: zero `println!` in `src/synthesis/`, no layer violations.

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
- Synthesis & UI (`src/synthesis/`): yes
- Config/oracles (`src/config/`, `src/oracles/`): no

## Acceptance Criteria

1. `generate_markdown_report(synthesis: &AuditSynthesis, branch: &str, squad: &str, sfd: Option<&SfdDocument>) -> String` generates formatted Markdown.
2. `write_markdown_report(report_content: &str, reports_dir: &Path) -> Result<PathBuf>` saves the report with timestamped filename and returns its path.
3. If reports directory does not exist, creates it automatically.
4. Unit tests in `src/synthesis/markdown.rs` verify report formatting, file persistence, and metadata inclusion.
5. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Create `src/synthesis/markdown.rs`.
- [x] Export `generate_markdown_report` and `write_markdown_report` in `src/synthesis/mod.rs`.
- [x] Add unit tests in `src/synthesis/markdown.rs`.
- [x] Wire report persistence into `src/cli/commands/audit.rs`.
- [x] Run `./scripts/verify.sh`.
- [x] Update `TODO.md`.

## Decision Log

- 2026-09-20: Use UTC timestamps in ISO 8601 / `YYYY-MM-DD-HHMMSS` format for collision-free report filenames.
- 2026-09-20: Decomposed `audit.rs` execution into focused helper functions to obey Clippy's `too-many-lines` pedantic lint.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
Outcome: All 71 tests passed (3 markdown tests, 2 terminal tests, 2 synthesis tests, 5 finding tests, 39 unittests, 6 mechanical architecture tests, 11 CLI tests, 3 init tests), 0 clippy warnings (including pedantic checks), release build passed.

## Completion Notes

Implemented `generate_markdown_report` and `write_markdown_report` saving persistent Markdown audit logs to `.argus/reports/`.
Phase 6 is now 100% completed.
All changes remain uncommitted in the working tree for review.
