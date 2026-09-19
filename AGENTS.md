# Argus Agent Guidelines

Argus is an autonomous, multi-agent adversarial pre-flight QA swarm orchestrator written in Rust.

## Operating Contract

- **DO NOT COMMIT OR PUSH UNLESS EXPLICITLY INSTRUCTED BY THE USER.** Leaving completed work uncommitted in the working tree is the strict default. Wait for an explicit user prompt before running any `git commit` or `git push`.
- When explicitly instructed to commit, use Conventional Commits: `type(scope): description`.
- Keep changes small, focused, idiomatic, and reversible.
- Prefer existing architecture, helpers, scripts, and Rust standards over new abstractions.
- Do not add speculative features, broad refactors, or unrequested configurability.
- If behavior, architecture, or public contracts change, update the relevant documentation.

## Source Of Truth

- Product Proposal: `PROPOSAL.md`
- Task Roadmap: `TODO.md`
- Architecture: `docs/architecture/application-architecture.md`
- Project Structure: `docs/architecture/project-structure.md`
- Design Principles: `docs/engineering/design-principles.md`
- Harness Design: `docs/engineering/harness-engineering-design.md`
- Guardrails: `docs/engineering/guardrails.md`
- Verification Loop: `docs/engineering/controlled_verification_loop.md`
- Testing Strategy: `docs/engineering/testing_strategy.md`
- Execution Plans: `docs/exec-plans/README.md`
- Tech Debt Tracker: `docs/exec-plans/tech-debt-tracker.md`

## Non-Negotiables

- **Zero Unhandled Panics:** Do not use `.unwrap()` or `.expect()` in non-test production logic. Use `?`, `Option::ok_or_else`, `Result`, and `anyhow::Context`.
- **Zero Unsafe Code:** Absolutely no `unsafe` blocks. Enforced mechanically via `#![forbid(unsafe_code)]`.
- **Zero Generic Names:** Modules and files named `helper`, `helpers`, `manager`, or `utils` are strictly forbidden. Enforced mechanically via `tests/architecture_test.rs`.
- **Deep Modules:** New interfaces must hide meaningful complexity from their callers. Avoid thin pass-through wrappers.
- **No Behavioral Mode Flags:** Avoid boolean flags that create dual behavioral modes; use distinct types or commands instead.
- **Fail-Safe Subprocess Management:** Subprocess management (`tokio::process::Command`) must handle non-zero exit codes, missing executables, and timeouts without crashing the orchestrator.
- **Bounded Concurrency:** Never spawn unbounded tasks. Always bound parallel `pi` subprocesses with `tokio::sync::Semaphore`.
- **Test Observable Invariants:** Test behavior and invariants at the lowest stable layer.
- **Execution Plans:** Use execution plans under `docs/exec-plans/active/` for non-trivial implementation work.
- **Mechanical Guardrails Over Memory:** If a failure mode, bug pattern, or review comment repeats 2+ times, promote it into a lint, compiler check, or architecture test.
- **Strict No-Commit Rule:** Never commit or push without explicit user instruction.

## Verification

Fast local gate:
```bash
cargo check
cargo test
```

Full local gate (all-in-one script):
```bash
./scripts/verify.sh
```

Manual checks:
```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo build --release
```

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
