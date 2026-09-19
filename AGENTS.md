# Argus Agent Guidelines

Argus is an autonomous, multi-agent adversarial pre-flight QA swarm orchestrator written in Rust.

## Operating Contract

- Keep changes small, focused, idiomatic, and reversible.
- Prefer existing architecture, helpers, scripts, and Rust standards over new abstractions.
- Do not add speculative features, broad refactors, or unrequested configurability.
- If behavior, architecture, or public contracts change, update the relevant documentation.
- Do not commit or push unless the user explicitly asks for a commit or push. Use Conventional Commits (`type(scope): description`) when committing.

## Source of Truth

- Project Proposal: `PROPOSAL.md`
- Task Roadmap: `TODO.md`
- Architecture: `discussions/12-argus-end-to-end-system-architecture.md`
- Standalone CLI Plan: `discussions/11-standalone-cli-architecture-and-implementation-plan.md`
- Persona Specification: `discussions/10-repository-standardization-and-dot-swarm-spec.md`
- Engineering Guardrails: `docs/engineering/guardrails.md`
- Verification Loop: `docs/engineering/controlled_verification_loop.md`
- Testing Strategy: `docs/engineering/testing_strategy.md`
- Execution Plans: `docs/exec-plans/README.md`

## Hard Rules (Rust Excellence)

- **Zero Unhandled Panics:** Do not use `.unwrap()` or `.expect()` in non-test / production logic. Use `?`, `Option::ok_or_else`, `Result`, and `anyhow::Context`.
- **Zero Unsafe Code:** Absolutely no `unsafe` blocks. Everything must remain 100% safe Rust.
- **Strict Compiler & Clippy Cleanliness:** Code must compile with zero warnings under `cargo clippy --all-targets --all-features -- -D warnings`.
- **Formatting Discipline:** Code must pass `cargo fmt --check` cleanly with 4-space Rust standard indentation.
- **Fail-Safe Process Management:** Subprocess management (`tokio::process::Command`) must handle non-zero exit codes, missing executables, and timeouts without crashing the orchestrator.
- **Bounded Concurrency:** Never spawn unbounded tasks. Use `tokio::sync::Semaphore` to cap concurrent `pi` subprocesses.
- **Immutable Pre-Existing Work:** Leave unrelated dirty worktree files untouched. Never run git operations that overwrite or revert user work.

## Architecture Map

```text
src/
├── main.rs                      # Binary entry point & CLI dispatch
├── cli/                         # Command-line interface definitions (clap)
│   ├── mod.rs
│   ├── args.rs                  # CLI flags & subcommand parsers
│   └── commands/                # Subcommand handlers (audit, init, personas, resume)
├── config/                      # .argus/config.yaml parser & models (serde)
├── git/                         # Git diff extraction & noise filtering
├── context/                     # SFD & specification ingestion
├── persona/                     # Two-tier persona registry & discovery
├── runner/                      # Native Pi subprocess launcher & pool (tokio)
├── synthesis/                   # Finding aggregator, ranker & report generators
└── oracles/                     # .argus/oracles.yaml parser & validator
```

## Agent Verification (Required)

Agents must verify changes before claiming completion using the canonical verification script or native cargo commands.

Fast local check:
```bash
cargo check
cargo test
```

Full gate:
```bash
./scripts/verify.sh
```
Or manually:
```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

## Harness Expectations

- Treat agent legibility as a repo-level quality goal: keep boundaries explicit, naming stable, and code easy to rediscover.
- For non-trivial tasks, follow the plan under `docs/exec-plans/active/`.
- If a failure mode, bug pattern, or review comment repeats 2+ times, promote it into:
  - a lint or compiler check
  - a persona prompt rule in `src/persona/builtin/`
  - an oracle rule in `.argus/oracles.yaml`
  - an engineering doc update
- Never claim a check passed without actually executing it and inspecting the command output.
