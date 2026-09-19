# Persona Definition and Frontmatter Parser

**Plan version:** 1  
**Task ID:** persona-definition-and-frontmatter-parser  
**Status:** active  
**Owner:** ahmad fikril  
**Risk:** low  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** src/persona/, src/main.rs, tests/, Cargo.toml, TODO.md  
**Allowed actions:** edit, verify  
**Maximum risk:** low  
**Repair limit:** 2  
**Task timeout:** 60m  

Date: 2026-09-19  
Related task: TODO.md Phase 4 Task 4.1  

## Objective

Implement the persona definition data structures and Markdown YAML frontmatter parser in `src/persona/frontmatter.rs`:
- Parse persona Markdown files containing YAML frontmatter using `gray_matter`.
- Extract metadata fields:
  - `name`: unique identifier (e.g. `stock-ledger-auditor`).
  - `title`: human-readable title.
  - `squad`: functional squad (`forms`, `state`, `sync`, `spec`, etc.).
  - `model_tier`: cost/intelligence tier (`fast`, `standard`, `deep`).
  - `tools`: comma-separated allowed tools (`read, grep, find, ls, bash`).
- Extract the Markdown body as the persona's system prompt instructions.
- Provide comprehensive validation and error messages for missing or invalid fields.

## Constraints

- Zero `unwrap()` or `expect()` in production paths.
- No generic file or module names (`helper`, `manager`, `utils`).
- Must pass `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- All changes must remain uncommitted in the working tree for user review.

## Impact Areas

- CLI commands (`src/cli/`): no
- Git & Diff engine (`src/git/`): no
- Context & SFD (`src/context/`): no
- Persona registry (`src/persona/`): yes
- Subprocess runner (`src/runner/`): no
- Synthesis & UI (`src/synthesis/`): no
- Config/oracles (`src/config/`, `src/oracles/`): no

## Acceptance Criteria

1. `Persona` struct holds parsed metadata:
   - `name: String`
   - `title: String`
   - `squad: String`
   - `model_tier: ModelTier` (enum: `Fast`, `Standard`, `Deep`)
   - `tools: Vec<String>`
   - `system_prompt: String`
   - `source: PersonaSource` (enum: `Builtin`, `ProjectOverride(PathBuf)`)
2. `parse_persona_markdown(content: &str, source: PersonaSource) -> Result<Persona>`:
   - Successfully extracts YAML frontmatter and Markdown body.
   - Trims and validates all fields.
   - Returns informative errors if required fields (`name`, `title`, `squad`) are missing or empty.
3. Unit tests cover valid frontmatter, missing fields, default tools, and model tier parsing.
4. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Create `src/persona/mod.rs` and `src/persona/frontmatter.rs`.
- [x] Implement `ModelTier`, `PersonaSource`, and `Persona` structs.
- [x] Implement `parse_persona_markdown`.
- [x] Add unit tests in `src/persona/frontmatter.rs`.
- [x] Wire `mod persona;` into `src/main.rs`.
- [x] Run `./scripts/verify.sh`.
- [x] Update `TODO.md`.

## Decision Log

- 2026-09-19: Use `gray_matter::Matter::<gray_matter::engine::YAML>::new()` for robust YAML frontmatter splitting and parsing.
- 2026-09-19: Default `tools` to `["read", "grep", "find", "ls", "bash"]` when omitted from frontmatter.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
Outcome: All 39 tests passed (4 persona frontmatter unit tests, 4 bundle tests, 6 SFD tests, 5 branch tests, 7 config tests, 3 architecture tests, 7 CLI tests, 3 init tests), 0 clippy warnings, release build passed.

## Completion Notes

Implemented `Persona`, `ModelTier`, and `PersonaSource` structs.
Implemented `parse_persona_markdown` parsing YAML frontmatter and extracting the Markdown body as system prompt instructions.
Covered with 4 unit tests verifying field deserialization, default values, missing name errors, and missing prompt body errors.
All changes remain uncommitted in the working tree for review.
