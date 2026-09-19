# Task Context and Payload Builder

**Plan version:** 1  
**Task ID:** task-context-and-payload-builder  
**Status:** active  
**Owner:** ahmad fikril  
**Risk:** low  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** src/context/, src/main.rs, tests/, Cargo.toml, TODO.md  
**Allowed actions:** edit, verify  
**Maximum risk:** low  
**Repair limit:** 2  
**Task timeout:** 60m  

Date: 2026-09-19  
Related task: TODO.md Phase 3 Task 3.2  

## Objective

Implement the task context and prompt payload builder in `src/context/bundle.rs`:
- Construct clean, deterministic prompt payloads passed to each subagent session.
- Instruct agents on the exact Git diff commands to run (`git diff`, `git diff --staged`, or `git diff <base>...HEAD`).
- Provide paths and references to the active Markdown SFD specification document when available.
- Enforce the structured output contract (status, file paths, line numbers, failure scenarios, and fixes).

## Constraints

- Zero `unwrap()` or `expect()` in production paths.
- No generic file or module names (`helper`, `manager`, `utils`).
- Must pass `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- All changes must remain uncommitted in the working tree for user review.

## Impact Areas

- CLI commands (`src/cli/`): no
- Git & Diff engine (`src/git/`): no
- Context & SFD (`src/context/`): yes
- Persona registry (`src/persona/`): no
- Subprocess runner (`src/runner/`): no
- Synthesis & UI (`src/synthesis/`): no
- Config/oracles (`src/config/`, `src/oracles/`): no

## Acceptance Criteria

1. `TaskContextBundle` struct encapsulates:
   - `branch: String`
   - `base_branch: Option<String>`
   - `staged_only: bool`
   - `sfd: Option<SfdDocument>`
2. `build_agent_prompt(bundle: &TaskContextBundle, persona_name: &str) -> String` generates a well-structured prompt instructing the subagent on:
   - What Git command to execute (`git diff`, `git diff --staged`, or `git diff <base>...HEAD`).
   - The path to the active SFD file (if present) and instruction to inspect it with `read`.
   - The reporting rubric for finding descriptions, file lines, and concrete fixes.
3. Unit tests cover all permutations: working tree changes, staged-only changes, base-branch comparison, with SFD, and without SFD.
4. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Create `src/context/bundle.rs`.
- [x] Export `TaskContextBundle` and `build_agent_prompt` in `src/context/mod.rs`.
- [x] Add unit tests in `src/context/bundle.rs`.
- [x] Wire payload generation into `src/main.rs`.
- [x] Run `./scripts/verify.sh`.
- [x] Update `TODO.md`.

## Decision Log

- 2026-09-19: Instruct subagents to run git diff natively via their bash tool, rather than piping raw diff text through CLI arguments, avoiding OS command-line argument length limits (`ARG_MAX`).

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
Outcome: All 35 tests passed (4 bundle unit tests, 6 SFD unit tests, 5 branch tests, 7 config tests, 3 architecture tests, 7 CLI tests, 3 init tests), 0 clippy warnings, release build passed.

## Completion Notes

Implemented `TaskContextBundle` and `build_agent_prompt` generating clear, structured prompt instructions for subagents.
Supports working tree changes, base branch comparisons (`--base`), staged-only changes (`--staged`), and optional active SFD references.
Phase 3 is now 100% completed.
All changes remain uncommitted in the working tree for review.
