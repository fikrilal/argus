# Auto-Persona Architect Initialization (`argus init --auto`)

**Plan version:** 1  
**Task ID:** auto-persona-architect-init  
**Status:** completed  
**Owner:** ahmad fikril  
**Risk:** medium  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** src/cli/, src/runner/, tests/, docs/, Cargo.toml  
**Allowed actions:** create, edit, verify  
**Maximum risk:** medium  
**Repair limit:** 2  
**Task timeout:** 90m  

Date: 2026-09-25  

## Objective

Implement the optional `--auto` flag for `argus init` to eliminate the cold-start problem when adopting Argus into a new repository:
1. When `argus init` is run without `--auto`, behavior remains 100% offline, instant, and deterministic.
2. When `argus init --auto` is executed:
   - Scaffolds the `.argus/` directory structure and starter configuration.
   - Spawns an autonomous **Persona Architect** subagent via the native Pi runner.
   - The architect agent explores the target repository, detects the language, framework, data layer, state management, and high-risk boundaries.
   - Synthesizes 3–5 tailored adversarial persona markdown files directly into `.argus/personas/<name>.md`.
   - Updates `.argus/config.yaml` with a tailored squad matching the project domain.
   - Provides live terminal spinner feedback during analysis and prints a clean summary of generated personas.

## Constraints

- Zero unhandled panics: no `.unwrap()` or `.expect()` in production paths.
- Zero unsafe code: enforced via `#![forbid(unsafe_code)]`.
- Zero generic file or module names: `helper`, `helpers`, `manager`, `utils` forbidden.
- Offline default: `argus init` without `--auto` must NEVER spawn subprocesses or make network calls.
- Hermetic testability: Must be verifiable in `tests/init_test.rs` via `ARGUS_PI_BIN` mock script.
- Strict no-commit rule: All changes remain uncommitted in the working tree for user review.

## Impact Areas

- CLI commands (`src/cli/`): yes (`args.rs`, `commands/init.rs`)
- Git & Diff engine (`src/git/`): no
- Context & SFD (`src/context/`): no
- Persona registry (`src/persona/`): no
- Subprocess runner (`src/runner/`): yes
- Synthesis & UI (`src/synthesis/`): no
- Config/oracles (`src/config/`): yes

## Acceptance Criteria

1. `argus init --help` documents `--auto`.
2. Standard `argus init` creates base directories and starter config without spawning subagents.
3. `argus init --auto` launches the `persona-architect` agent using `runner::launch_agent`.
4. The architect subagent receives an explicit system prompt and operational instructions to create valid Argus personas matching frontmatter schema (`name`, `title`, `squad`, `model_tier`, `tools`).
5. Hermetic integration tests in `tests/init_test.rs` test `argus init --auto` using a mock script (`ARGUS_PI_BIN`).
6. All verification gates in `./scripts/verify.sh` pass cleanly with zero warnings.

## Implementation Checklist

- [x] Add `--auto` flag to `InitArgs` in `src/cli/args.rs`.
- [x] Create embedded `persona-architect` persona definition and prompt builder in `src/runner/architect.rs`.
- [x] Implement auto-profiling orchestration in `src/cli/commands/init.rs`:
  - Show live spinner using `indicatif`.
  - Execute `persona-architect` agent targeting `target_dir`.
  - Scan `.argus/personas/` for generated files and print clean summary.
  - Register generated personas under squad `auto` in `.argus/config.yaml`.
- [x] Add integration test in `tests/init_test.rs` verifying `--auto` flag with mock `ARGUS_PI_BIN`.
- [x] Update `TODO.md` roadmap.
- [x] Run `./scripts/verify.sh`.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
