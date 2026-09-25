# CLI Option Conflicts and Subcommand Decoupling

**Plan version:** 1  
**Task ID:** cli-conflicts-and-subcommand-decoupling  
**Status:** completed  
**Owner:** ahmad fikril  
**Risk:** low  
**Authority:** implement and verify locally; no external mutation  
**Allowed paths:** src/cli/, src/config/, src/persona/, src/main.rs, tests/  
**Allowed actions:** edit, verify  
**Maximum risk:** low  
**Repair limit:** 2  
**Task timeout:** 60m  

Date: 2026-09-20  
Origin: Argus dogfooding audit (`.argus/reports/2026-10-01-234141-audit.md` findings #1, #7, #8, #9, #10, #11, #12)

## Objective

Remediate CLI argument conflicts, configuration discovery, and subcommand initialization coupling:
1. In `src/cli/args.rs`, add `conflicts_with` constraints between mutually exclusive flags (`--full`, `--staged`, `--base`).
2. Add support for feature/directory scoping (`--path <dir>`) so users can audit specific modules/subsystems.
3. In `src/main.rs`, decouple configuration and persona registry loading so `argus init` executes even if local `.argus/config.yaml` has syntax errors or missing files. Add error context on `env::current_dir()`.
4. In `src/config/loader.rs`, traverse parent directories (`dir.ancestors()`) so running `argus` from a project subdirectory reliably discovers `.argus/config.yaml`.
5. In `src/config/loader.rs`, handle directory paths passed to `--config` by searching for candidate config files within that directory.
6. In `src/persona/registry.rs`, enforce strict priority (`.argus` > `.swarm`).
7. In `src/persona/registry.rs`, ensure `--squad all` dynamically resolves novel personas added to `.argus/personas/`.

## Constraints

- Zero `unwrap()` or `expect()` in production paths.
- No generic file or module names (`helper`, `manager`, `utils`).
- Must pass `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`, and `cargo test`.
- All changes must remain uncommitted in the working tree for user review.

## Impact Areas

- CLI commands (`src/cli/`): yes
- Git & Diff engine (`src/git/`): no
- Context & SFD (`src/context/`): no
- Persona registry (`src/persona/`): yes
- Subprocess runner (`src/runner/`): no
- Synthesis & UI (`src/synthesis/`): no
- Config/oracles (`src/config/`, `src/oracles/`): yes

## Acceptance Criteria

1. Running `argus audit --full --staged` or `argus audit --staged --base main` fails immediately with a Clap conflict error.
2. Running `argus init --force` succeeds even when `.argus/config.yaml` contains invalid YAML.
3. Running `argus` from a subdirectory (e.g. `src/cli/`) correctly loads `.argus/config.yaml` from project root.
4. If a custom persona is added to `.argus/personas/`, running with `--squad all` includes the custom persona.
5. All verification gates in `./scripts/verify.sh` pass cleanly.

## Implementation Checklist

- [x] Add `conflicts_with` attributes in `src/cli/args.rs`.
- [x] Add `--path` option in `src/cli/args.rs` and wire into `TaskContextBundle`.
- [x] Decouple `main.rs` subcommand loading.
- [x] Implement ancestor traversal in `src/config/loader.rs`.
- [x] Refine `.argus` priority and dynamic `all` squad in `src/persona/registry.rs`.
- [x] Add unit tests in `src/config/tests.rs` and `src/persona/registry.rs`.
- [x] Run `./scripts/verify.sh`.

## Verification

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```
