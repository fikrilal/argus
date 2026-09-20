# Lead Synthesizer and Deduplication

**Plan version:** 1  
**Task ID:** lead-synthesizer-and-deduplication  
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
Related task: TODO.md Phase 6 Task 6.2  

## Objective

Implement the output aggregator and findings deduplication engine in `src/synthesis/synthesizer.rs`:
- Aggregate raw agent results (`Vec<AgentRunResult>`) and extract structured findings using `parse_agent_findings`.
- Deduplicate overlapping findings when multiple agents report the same defect on the same target file and line.
- When deduplicating, elevate to the highest severity and merge reporting personas.
- Sort findings strictly by severity (`P0Blocker` -> `P1Major` -> `P2Polish`), then by file path and line number.
- Produce an `AuditSynthesis` summary containing total findings, severity breakdowns, and list of passing personas.

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

1. `AuditSynthesis` struct holds:
   - `findings: Vec<Finding>`
   - `total_agents: usize`
   - `successful_agents: usize`
   - `passed_personas: Vec<String>`
   - `blocker_count: usize`
   - `major_count: usize`
   - `polish_count: usize`
   - Method `is_passed() -> bool` (true if blocker_count == 0 && major_count == 0).
2. `synthesize_results(results: &[AgentRunResult]) -> AuditSynthesis`:
   - Aggregates findings from all agent outputs.
   - Merges duplicate findings targeting the same file and line, combining personas and retaining the highest severity.
   - Sorts findings by severity (P0 first) and file location.
3. Unit tests cover deduplication of overlapping agent findings, sorting order, and synthesis metrics.
4. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Create `src/synthesis/synthesizer.rs`.
- [x] Export `AuditSynthesis` and `synthesize_reports` in `src/synthesis/mod.rs`.
- [x] Add unit tests in `src/synthesis/synthesizer.rs`.
- [x] Wire synthesis into `src/cli/commands/audit.rs`.
- [x] Run `./scripts/verify.sh`.
- [x] Update `TODO.md`.

## Decision Log

- 2026-09-20: Deduplicate based on `(file_path, line_number)` when line number is present, or `(file_path, title)` when line number is absent.
- 2026-09-20: Elevate severity to the highest observed and concatenate reporting persona names on duplicate merges.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
Outcome: All 66 tests passed (2 synthesis tests, 5 finding tests, 39 unittests, 6 mechanical architecture tests, 11 CLI tests, 3 init tests), 0 clippy warnings (including pedantic checks), release build passed.

## Completion Notes

Implemented `AuditSynthesis`, `AgentReport`, and `synthesize_reports`.
Deduplicates multiple agent observations on the same file/line, elevating severity and merging personas.
Sorts findings strictly: P0 -> P1 -> P2 -> file path -> line number.
All changes remain uncommitted in the working tree for review.
