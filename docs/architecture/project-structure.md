# Argus Project Structure

## Status

Approved on October 1, 2026.

This document translates the application architecture into a repository layout, module ownership boundaries, and dependency rules.

## Structural Principles

- Single Rust binary crate initially.
- Strictly domain-driven modules inside `src/`.
- No generic dumping grounds: `helper`, `helpers`, `manager`, and `utils` are strictly forbidden and enforced by `tests/architecture_test.rs`.
- Clean unidirectional dependency flow:
  `cli` $\to$ `synthesis` / `runner` / `context` $\to$ `persona` / `config` / `git` / `oracles`.
- Integration and architecture tests live in `tests/`.

## Repository Layout

```text
argus/
├── Cargo.toml                       # Package manifest, lints, and dependencies
├── Cargo.lock
├── rust-toolchain.toml              # Pinned Rust 1.95.0 + clippy + rustfmt
├── LICENSE                          # MIT License
├── README.md                        # Documentation & entry point
├── PROPOSAL.md                      # Consolidated Engineering Proposal
├── TODO.md                          # Implementation roadmap & task breakdown
├── AGENTS.md                        # Operating contract & agent guidelines
├── discussions/                     # Architectural design records (01-12)
├── docs/                            # Deep engineering documentation
│   ├── architecture/
│   │   ├── application-architecture.md
│   │   └── project-structure.md
│   ├── engineering/
│   │   ├── design-principles.md
│   │   ├── harness-engineering-design.md
│   │   ├── guardrails.md
│   │   ├── testing-strategy.md
│   │   └── controlled-verification-loop.md
│   └── exec-plans/
│       ├── README.md
│       ├── _template.md
│       ├── tech-debt-tracker.md
│       ├── active/
│       └── completed/
├── scripts/
│   └── verify.sh                    # One-command verification gate
├── tests/
│   └── architecture_test.rs         # Mechanical architecture and safety tests
└── src/
    ├── main.rs                      # Binary entry point & CLI dispatch
    ├── cli/                         # Command-line interface definitions (clap)
    ├── config/                      # .argus/config.yaml parser & models (serde)
    ├── git/                         # Git diff extraction & noise filtering
    ├── context/                     # SFD & specification ingestion
    ├── persona/                     # Two-tier persona registry & discovery
    ├── runner/                      # Native Pi subprocess launcher & pool (tokio)
    ├── synthesis/                   # Finding aggregator, ranker & report generators
    └── oracles/                     # .argus/oracles.yaml parser & validator
```

## Module Ownership & Boundary Rules

1. `src/cli/` owns CLI argument parsing, flags, and command routing. It must not contain business logic or subprocess execution details.
2. `src/git/` owns interacting with the local Git binary, branch detection, and diff noise stripping.
3. `src/persona/` owns persona frontmatter parsing, embedded Tier-1 personas, and project-level `.argus/personas/` overrides.
4. `src/runner/` owns spawning asynchronous `tokio::process::Command` processes, managing concurrency semaphores, and streaming live progress. It must not depend on CLI or presentation modules.
5. `src/synthesis/` owns defect severity ranking (P0/P1/P2), terminal dashboard rendering, and Markdown report generation.
