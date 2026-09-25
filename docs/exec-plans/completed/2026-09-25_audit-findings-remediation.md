# Audit Findings Remediation

**Plan version:** 1  
**Task ID:** audit-findings-remediation  
**Status:** completed  
**Owner:** ahmad fikril  
**Risk:** medium  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** src/, tests/, Cargo.toml, docs/  
**Allowed actions:** edit, verify  
**Maximum risk:** medium  
**Repair limit:** 2  
**Task timeout:** 90m  

Date: 2026-09-25  
Origin: Argus self-audit report (`.argus/reports/2026-10-02-150043-audit.md` findings #3, #4, #5, #6, #7, #8, #9, #10, #11)

## Objective

Remediate all remaining findings from `.argus/reports/2026-10-02-150043-audit.md` in order:
1. **Finding #5, #4, #3:** Encapsulate recursion prevention in `src/runner/ancestry.rs` (moving low-level OS process hierarchy traversal out of `src/cli/`). Fix fragile whitespace parsing by using `stat.rfind(')')` to handle process names with spaces. Provide platform fallback for non-Linux OSes.
2. **Finding #6:** Move direct `tokio::process::Command` execution from `src/cli/commands/resume.rs` into `src/runner/session.rs`, respecting module ownership rules.
3. **Finding #7:** In `src/config/loader.rs`, resolve relative `custom_path` arguments against `start_dir` rather than process CWD.
4. **Finding #8:** In `src/config/loader.rs`, search for `config.yaml` / `config.yml` directly in directory paths passed to `--config`.
5. **Finding #9:** In `src/persona/registry.rs`, traverse `project_root.ancestors()` so `.argus/personas/` overrides are discovered from subdirectories.
6. **Finding #10:** In `src/persona/registry.rs`, eliminate brittle magic numbers (`self.len() > 12 && names.len() == 11`) in `get_squad("all", ...)`.
7. **Finding #11:** In `src/synthesis/synthesizer.rs`, require an explicit `[STATUS: PASS]` marker before recording a persona in `passed_personas`, preventing malformed/empty outputs from registering as clean passes.

## Constraints

- Zero `unwrap()` or `expect()` in production paths.
- No generic file or module names (`helper`, `helpers`, `manager`, `utils`).
- Must pass `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- All changes must remain uncommitted in the working tree for user review until explicitly requested.

## Impact Areas

- CLI commands (`src/cli/`): yes (`audit.rs`, `resume.rs`)
- Git & Diff engine (`src/git/`): no
- Context & SFD (`src/context/`): no
- Persona registry (`src/persona/`): yes (`registry.rs`)
- Subprocess runner (`src/runner/`): yes (`ancestry.rs`, `session.rs`, `mod.rs`)
- Synthesis & UI (`src/synthesis/`): yes (`synthesizer.rs`)
- Config/oracles (`src/config/`): yes (`loader.rs`, `tests.rs`)

## Acceptance Criteria

1. Recursion check lives in `src/runner/ancestry.rs` and properly parses Linux `/proc` stats when process names contain spaces.
2. `resume.rs` delegates subprocess resumption to `runner::resume_interactive_session`.
3. `load_config(Some(Path::new("relative/config.yaml")), start_dir)` resolves against `start_dir`.
4. `load_config(Some(Path::new(".argus")), start_dir)` discovers `config.yaml` inside `.argus`.
5. `PersonaRegistry::load` discovers `.argus/personas/` when invoked from project subdirectories.
6. Squad `"all"` resolution dynamically incorporates custom personas without hardcoded integer counts.
7. Unparseable/empty agent reports are not recorded in `passed_personas`.
8. All unit, integration, and architecture tests pass cleanly.

## Implementation Checklist

- [x] Create `src/runner/ancestry.rs` with robust `/proc` parsing and wire into `src/cli/commands/audit.rs` (Findings #3, #4, #5).
- [x] Implement `runner::resume_interactive_session` in `src/runner/session.rs` and delegate from `src/cli/commands/resume.rs` (Finding #6).
- [x] Fix relative path resolution in `src/config/loader.rs` and add tests (Finding #7).
- [x] Support direct directory config candidate resolution in `src/config/loader.rs` and add tests (Finding #8).
- [x] Implement ancestor traversal in `src/persona/registry.rs` and add tests (Finding #9).
- [x] Replace magic numbers in `src/persona/registry.rs` with clean dynamic worker inclusion (Finding #10).
- [x] Enforce explicit pass marker check in `src/synthesis/synthesizer.rs` and add tests (Finding #11).
- [x] Run `./scripts/verify.sh`.
