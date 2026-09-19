# SFD Specification Reader

**Plan version:** 1  
**Task ID:** sfd-specification-reader  
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
Related task: TODO.md Phase 3 Task 3.1  

## Objective

Implement the System Functional Design (SFD) specification reader in `src/context/sfd.rs`:
- Locate the active Markdown SFD via explicit CLI argument (`--sfd <path>`) or project configuration (`.argus/config.yaml`).
- Read and validate the file: ensure it exists, is accessible, and is non-empty.
- Extract document title or metadata if available.
- Gracefully handle projects where no SFD is supplied (returns `Ok(None)`), allowing audits to run without requiring an SFD.

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

1. `SfdDocument` struct holds `path: PathBuf`, `content: String`, and extracted `title: Option<String>`.
2. `load_sfd(explicit_path: Option<&Path>, config: &ArgusConfig, project_root: &Path) -> Result<Option<SfdDocument>>`:
   - Prioritizes `explicit_path` if provided.
   - Falls back to `config.sfd.active` located inside `config.sfd.dir`.
   - Returns `Ok(None)` if neither is specified.
   - Returns clear `Err` if a specified file does not exist or is empty.
3. Unit tests cover explicit path loading, configured path loading, missing file error, empty file error, and optional none.
4. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Create `src/context/mod.rs` and `src/context/sfd.rs`.
- [x] Implement `SfdDocument` and `load_sfd`.
- [x] Add unit tests in `src/context/sfd.rs`.
- [x] Wire `src/context/` into `src/main.rs`.
- [x] Run `./scripts/verify.sh`.
- [x] Update `TODO.md`.

## Decision Log

- 2026-09-19: Return `Option<SfdDocument>` instead of forcing an SFD to be present so Argus remains universal for repos without formal SFDs.
- 2026-09-19: Extract first Markdown `# Title` heading as human-friendly specification name.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
Outcome: All 31 tests passed (6 SFD tests, 5 branch tests, 7 config tests, 3 architecture tests, 7 CLI tests, 3 init tests), 0 clippy warnings, release build passed.

## Completion Notes

Implemented Markdown SFD specification reader in `src/context/sfd.rs`.
Supports `--sfd <path>` override and `.argus/config.yaml` active file resolution.
Includes title extraction and non-empty file validation.
All changes remain uncommitted in the working tree for review.
