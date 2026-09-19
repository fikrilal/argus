# Branch Detection and Session Slugification

**Plan version:** 1  
**Task ID:** branch-detection-and-session-slugification  
**Status:** active  
**Owner:** ahmad fikril  
**Risk:** low  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** src/git/, src/main.rs, tests/, Cargo.toml, TODO.md  
**Allowed actions:** edit, verify  
**Maximum risk:** low  
**Repair limit:** 2  
**Task timeout:** 60m  

Date: 2026-09-19  
Related task: TODO.md Phase 2 Task 2.1  

## Objective

Implement the Git branch detection and session slugification module in `src/git/branch.rs`:
- Detect the current Git branch asynchronously using `tokio::process::Command` (`git rev-parse --abbrev-ref HEAD`).
- Handle detached HEAD states gracefully by falling back to commit hash prefix (`detached-<sha>`).
- Implement `slugify_branch_name` to convert branches with slashes and special characters (e.g. `DEV/AFM/AUTH_MIGRATION`) into safe, clean session tokens (`DEV-AFM-AUTH-MIGRATION`).

## Constraints

- Zero `unwrap()` or `expect()` in production paths.
- No generic file or module names (`helper`, `manager`, `utils`).
- Must use asynchronous `tokio::process::Command` (no blocking `std::process::Command`).
- Must pass `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- All changes must remain uncommitted in the working tree for user review.

## Impact Areas

- CLI commands (`src/cli/`): no
- Git & Diff engine (`src/git/`): yes
- Persona registry (`src/persona/`): no
- Subprocess runner (`src/runner/`): no
- Synthesis & UI (`src/synthesis/`): no
- Config/oracles (`src/config/`, `src/oracles/`): no

## Acceptance Criteria

1. `detect_current_branch(repo_root: &Path) -> Result<String>` asynchronously resolves the active Git branch.
2. In detached HEAD state, it resolves to `detached-<short_hash>`.
3. `slugify_branch_name(branch: &str) -> String` produces clean, alphanumeric-and-hyphen slugs:
   - Slashes (`/`), backslashes (`\`), spaces, and unsafe punctuation are replaced with hyphens (`-`).
   - Consecutive hyphens are collapsed into one.
   - Leading and trailing hyphens are stripped.
   - Example: `DEV/AFM/AUTH_MIGRATION` $\to$ `DEV-AFM-AUTH_MIGRATION` (or `DEV-AFM-AUTH-MIGRATION`).
4. Unit tests cover standard branches, nested slash branches, detached states, and edge cases.
5. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Create `src/git/mod.rs` and `src/git/branch.rs`.
- [x] Implement `slugify_branch_name`.
- [x] Implement `detect_current_branch` with fallback for detached HEAD.
- [x] Add unit tests in `src/git/branch.rs`.
- [x] Wire `src/git/` into `src/main.rs`.
- [x] Run `./scripts/verify.sh`.
- [x] Update `TODO.md`.

## Decision Log

- 2026-09-19: Use `tokio::process::Command` to adhere to our non-blocking runner guardrail.
- 2026-09-19: Preserve underscores in slugified branch names (`_` is safe in session names and common in branch tickets), but replace `/`, `\`, spaces, `#`, `@`, `:`, etc. with `-`.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
Outcome: All 25 tests passed (5 branch unit tests, 7 config tests, 3 architecture tests, 7 CLI tests, 3 init tests), 0 clippy warnings, release build passed.

## Completion Notes

Implemented async Git branch detection and deterministic session slugification.
Added unit tests covering complex nested branches (e.g. `DEV/AFM/AUTH_MIGRATION` -> `DEV-AFM-AUTH_MIGRATION`), special character stripping, consecutive dash collapse, and fallback to `main`.
All changes remain uncommitted in the working tree for review.
