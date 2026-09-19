# 11 — Standalone CLI Architecture & Implementation Plan (Rust)

**Document Status:** Approved Decision Record  
**Target Domain:** Standalone Rust CLI Package Design & Component Implementation Plan  
**Binary Name:** `argus`  
**Repository:** `fikrilal/argus`  
**Date:** October 2026  

---

## 1. Decision: Standalone Rust CLI over Monorepo Bundling

A critical architectural decision was finalized: **Argus will be developed as an independent, standalone Rust CLI package (`argus`)**, completely decoupled from `mobile-core-kit` or any specific application repository.

### Rationale:
1. **Zero Runtime Friction (Single Static Binary):** Compiles to a single, statically linked native binary (`target/release/argus`). Runs on any developer machine without requiring Node.js, Dart, Go, or Python runtimes.
2. **Sub-Millisecond Startup & Minimal Memory Footprint:** Starts in $< 3\text{ms}$ and consumes $< 10\text{MB}$ RAM, making it suitable for both local development and CI/CD pipelines.
3. **World-Class CLI Ecosystem:** Leverages the undisputed best CLI and async ecosystems in the industry:
   - `clap` (derive API) for typed command-line interfaces and shell completions.
   - `tokio` for concurrent subprocess management and stream piping.
   - `indicatif` for multi-progress bars and concurrent agent spinners.
   - `serde` / `serde_yaml` for resilient YAML frontmatter and config parsing.
4. **Universal Portability:** Inspects Flutter, React Native, Swift, Kotlin, Go, or Python codebases with equal fidelity.

---

## 2. Package Identity & Structure

- **Crate Name:** `argus`
- **CLI Executable:** `argus`
- **GitHub Repository:** `https://github.com/fikrilal/argus`
- **Location:** `/home/fikrilal/workspace/devs/experiments/argus`

### Directory Layout:

```text
argus/
├── Cargo.toml                       # Package manifest & dependencies
├── Cargo.lock
├── LICENSE                          # MIT License
├── README.md                        # Documentation & overview
├── PROPOSAL.md                      # Consolidated Engineering Proposal
├── discussions/                     # Architectural design records (01-11)
└── src/
    ├── main.rs                      # Binary entry point & CLI dispatch
    ├── cli/                         # Command-line interface definitions (clap)
    │   ├── mod.rs
    │   ├── args.rs                  # Top-level CLI arguments & subcommands
    │   └── commands/
    │       ├── mod.rs
    │       ├── audit.rs             # `argus audit` workflow
    │       ├── init.rs              # `argus init` scaffolding
    │       ├── personas.rs          # `argus personas` listing & inspection
    │       └── resume.rs            # `argus resume` interactive session picker
    ├── config/                      # .argus/config.yaml parser & models (serde)
    │   ├── mod.rs
    │   ├── schema.rs
    │   └── loader.rs
    ├── git/                         # Git interaction & diff noise filtering
    │   ├── mod.rs
    │   ├── diff.rs                  # Git diff capture & noise stripping
    │   └── branch.rs                # Branch name detection & slugification
    ├── context/                     # SFD & project specification ingestion
    │   ├── mod.rs
    │   ├── sfd.rs                   # Markdown SFD reader & section parser
    │   └── bundle.rs                # Assembled prompt payload bundle
    ├── persona/                     # Two-tier persona registry & discovery
    │   ├── mod.rs
    │   ├── frontmatter.rs           # YAML frontmatter parser
    │   ├── builtin.rs               # Embedded Tier-1 base personas (include_str!)
    │   └── registry.rs              # Two-tier persona resolver (.argus/personas/)
    ├── runner/                      # Native Pi subprocess orchestration (tokio)
    │   ├── mod.rs
    │   ├── launcher.rs              # Process spawn (`pi -p ...`)
    │   ├── session.rs               # Deterministic session naming (`argus/<branch>/<persona>`)
    │   └── pool.rs                  # Concurrency limiter & task scheduler
    ├── synthesis/                   # Aggregation, severity ranking & UI
    │   ├── mod.rs
    │   ├── finding.rs               # Defect finding data structures
    │   ├── ranker.rs                # P0 / P1 / P2 severity triaging
    │   ├── terminal.rs              # Colorized terminal dashboard renderer
    │   └── markdown.rs              # Output Markdown report generator
    └── oracles/                     # .argus/oracles.yaml parser & validator
        ├── mod.rs
        └── registry.rs
```

---

## 3. Core Implementation Modules

### Module 1: `git::diff` (Diff Extraction & Noise Stripping)
- Captures `git diff` against base branches (`origin/main`, `origin/development`, or working tree).
- **Noise Filter:** Automatically strips out generated files (`*.g.dart`, `*.freezed.dart`, `pubspec.lock`, `Cargo.lock`, `package-lock.json`, build artifacts) to maximize LLM reasoning bandwidth on human-written logic.

### Module 2: `persona::registry` (Two-Tier Persona Engine)
- Discovers and loads personas with **Two-Tier Inheritance**:
  - Embedded Tier-1 base personas (compiled directly into the binary via Rust's `include_str!`).
  - Project-local Tier-2 personas loaded from `<repo>/.argus/personas/*.md` (or `.swarm/personas/*.md`).
- Parses YAML frontmatter (`name`, `title`, `squad`, `model_tier`, `tools`).

### Module 3: `runner::launcher` (Native Pi Process Spawner)
- Launches native `pi` subprocesses via `tokio::process::Command`.
- Enforces deterministic session naming: `argus/<branch-slug>/<persona-slug>`.
- Controls concurrency (e.g. running 4 worker sessions in parallel using Tokio semaphore).
- Non-interactive execution (`-p`) that streams progress while persisting session state for subsequent `pi -r` resumption.

### Module 4: `context::sfd` (SFD Specification Ingestion)
- Ingests converted Markdown SFD specifications.
- Bundles the SFD with the git diff and injects it into personas that declare SFD context requirements (e.g. `sfd-clause-detective`).

### Module 5: `synthesis::terminal` & `synthesis::markdown`
- Aggregates findings from worker sessions.
- Formats terminal output with visual severity tags:
  - `[BLOCKER / P0]`: Data loss, stock ledger imbalance, unhandled crashes on HTTP 200 OK.
  - `[MAJOR / P1]`: Unsanitized form inputs, bypassed business validations, cascade dropdown errors.
  - `[POLISH / P2]`: Minor formatting, missing soft limits, cosmetic feedback.
- Writes a Markdown summary to `.argus/reports/<timestamp>-audit.md`.

---

## 4. Dependencies (`Cargo.toml`)

```toml
[package]
name = "argus"
version = "0.1.0"
edition = "2024"
authors = ["ahmad fikril <fikrildev@gmail.com>"]
description = "The all-seeing pre-flight QA swarm. Catches edge cases, state invariant leaks, and spec gaps before human QA."
license = "MIT"
repository = "https://github.com/fikrilal/argus"

[dependencies]
clap = { version = "4.5", features = ["derive", "cargo", "env"] }
tokio = { version = "1.38", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
indicatif = { version = "0.17", features = ["tokio"] }
colored = "2.1"
gray_matter = "0.2"
anyhow = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
walkdir = "2.5"
regex = "1.10"
chrono = { version = "0.4", features = ["serde"] }

[dev-dependencies]
tempfile = "3.10"
assert_cmd = "2.0"
predicates = "3.1"
```

---

## 5. Development Phases

1. **Phase 1: Project Scaffolding & CLI Skeleton:** Setup `Cargo.toml`, `src/main.rs`, and `clap` subcommands (`audit`, `init`, `personas`, `resume`).
2. **Phase 2: Git Diff & Persona Registry:** Implement `git::diff` with noise filtering, and `persona::registry` with two-tier resolution.
3. **Phase 3: Pi Async Orchestrator:** Implement `runner::launcher` with Tokio subprocess management, deterministic session naming, and `indicatif` progress streaming.
4. **Phase 4: Synthesis & Output Formatting:** Implement Lead Synthesizer aggregation, terminal color tables, and Markdown report persistence.
5. **Phase 5: End-to-End Validation:** Run against `superapps` branch `DEV/AFM/AUTH_MIGRATION` to verify real-world edge case detection.
