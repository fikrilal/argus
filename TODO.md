# Argus Implementation Roadmap & Task Breakdown (TODO)

This document defines the atomic, bite-sized implementation chunks for **Argus**.  
Each task follows the strict execution discipline: **Execute $\to$ Review $\to$ Test $\to$ Commit**.

---

## Progress Overview

- [x] **Phase 1: Foundation, CLI Skeleton & Configuration** (Tasks 1.1 – 1.4)
- [x] **Phase 2: Git Engine & Session Identity** (Task 2.1)
- [x] **Phase 3: Context & SFD Ingestion** (Tasks 3.1 – 3.2)
- [x] **Phase 4: Two-Tier Persona Engine** (Tasks 4.1 – 4.4)
- [x] **Phase 5: Native Pi Subprocess Runner & Concurrency Pool** (Tasks 5.1 – 5.4)
- [ ] **Phase 6: Lead Synthesis & Output Reporting** (Tasks 6.1 – 6.4)
- [ ] **Phase 7: Interactive CLI Commands & End-to-End Verification** (Tasks 7.1 – 7.3)

---

## Phase 1: Foundation, CLI Skeleton & Configuration

### Task 1.1: Dependencies & Project Manifest (`Cargo.toml`)
- [x] Add core crates: `clap` (derive, cargo, env), `tokio` (full), `serde`, `serde_yaml`, `colored`, `indicatif`, `anyhow`, `gray_matter`, `walkdir`, `regex`, `chrono`.
- [x] Configure release build profile (`opt-level = 3`, `lto = true`, `strip = true`).
- [x] **Verification:** `cargo check` passes cleanly.
- [x] **Commit:** `chore(cargo): configure dependencies and release profile`

### Task 1.2: CLI Subcommands & Argument Parsing (`src/cli/`)
- [x] Create `src/cli/args.rs` with `clap` derive structs:
  - Top-level `Cli` with global flags (`--verbose`, `--config`).
  - Subcommands: `Audit`, `Init`, `Personas`, `Resume`.
- [x] Wire entry point in `src/main.rs`.
- [x] **Verification:** `cargo run -- --help` prints formatted help with all subcommands.
- [x] **Commit:** `feat(cli): define top-level command-line interface and subcommands`

### Task 1.3: Project Configuration Model & Loader (`src/config/`)
- [x] Create `src/config/schema.rs` with `ArgusConfig`, `SfdConfig`, `ModelTiersConfig`, and `SquadConfig`.
- [x] Create `src/config/loader.rs` to locate and parse `.argus/config.yaml` (or fallback to default config).
- [x] Unit tests for YAML deserialization and default fallbacks.
- [x] **Verification:** `cargo test config` passes.
- [x] **Commit:** `feat(config): implement .argus/config.yaml parser and schema models`

### Task 1.4: Scaffolding Command (`argus init`)
- [x] Implement `src/cli/commands/init.rs`:
  - Scaffolds `.argus/` directory tree (`.argus/personas/`, `.argus/context/sfd/`, `.argus/reports/`).
  - Generates starter `.argus/config.yaml` and starter `.argus/oracles.yaml`.
  - Idempotent: warns if `.argus/` already exists without overwriting existing files.
- [x] Unit test in a temporary directory (`tempfile`).
- [x] **Verification:** `cargo test init` passes; running `argus init` creates valid files.
- [x] **Commit:** `feat(cli): implement argus init scaffolding command`

---

## Phase 2: Git Engine & Session Identity

### Task 2.1: Branch Detection & Session Slugification (`src/git/branch.rs`)
- [x] Detect current Git branch via `git rev-parse --abbrev-ref HEAD`.
- [x] Implement slugifier: convert `DEV/AFM/AUTH_MIGRATION` $\to$ `DEV-AFM-AUTH_MIGRATION` (safe for session names and paths).
- [x] Unit tests for slash and special character sanitization.
- [x] **Verification:** `cargo test git::branch` passes.
- [x] **Commit:** `feat(git): add branch detection and session slugification`

*(Note: Custom in-process diff parsing/filtering was intentionally eliminated in favor of delegating `git diff` inspection directly to Pi agents via their native bash/read tools, keeping the orchestrator lean and zero-maintenance).*

---

## Phase 3: Context & SFD Ingestion

### Task 3.1: SFD Specification Reader (`src/context/sfd.rs`)
- [x] Locate and read active Markdown SFD specified in `.argus/config.yaml` or `--sfd <path>`.
- [x] Validate file exists and is non-empty.
- [x] Provide helper to read SFD content or path.
- [x] Unit tests for missing, valid, and custom SFD paths.
- [x] **Verification:** `cargo test context::sfd` passes.
- [x] **Commit:** `feat(context): implement Markdown SFD specification reader`

### Task 3.2: Task Context & Payload Builder (`src/context/bundle.rs`)
- [x] Construct the evaluation task prompt passed to subagents:
  - Injects target branch and base diff instructions (`--base` or `--staged`).
  - Injects reference to active SFD file.
  - Generates concise instructions for the agent to execute `git diff` and evaluate against its rubric.
- [x] Unit tests for payload generation.
- [x] **Verification:** `cargo test context::bundle` passes.
- [x] **Commit:** `feat(context): implement task context and payload builder`
- [ ] **Verification:** `cargo test context::bundle` passes.
- [ ] **Commit:** `feat(context): implement context payload bundler for agent dispatch`

---

## Phase 4: Two-Tier Persona Engine

### Task 4.1: Persona Definition & Frontmatter Parser (`src/persona/frontmatter.rs`)
- [x] Parse Markdown with YAML frontmatter using `gray_matter`:
  - `name`: string identifier (e.g. `stock-ledger-auditor`)
  - `title`: human-readable description
  - `squad`: `forms` | `state` | `sync` | `spec`
  - `model_tier`: `fast` | `standard` | `deep`
  - `tools`: comma-separated tool list (`read, grep, find, ls, bash`)
- [x] System prompt body extraction.
- [x] Unit tests for valid and invalid frontmatter.
- [x] **Verification:** `cargo test persona::frontmatter` passes.
- [x] **Commit:** `feat(persona): implement persona frontmatter parser and validation`

### Task 4.2: Embedded Tier-1 Base Personas (`src/persona/builtin.rs`)
- [x] Embed default personas directly into the binary using `include_str!`:
  - `sfd-clause-detective.md`
  - `id-regulatory-sentinel.md`
  - `rbac-identity-gatekeeper.md`
  - `form-boundary-saboteur.md`
  - `cascade-dropdown-glitcher.md`
  - `concurrency-double-tapper.md`
  - `stock-ledger-auditor.md`
  - `orphan-cascade-hunter.md`
  - `payload-pessimist.md`
  - `sync-deadlock-guard.md`
  - `hardware-sensor-adversary.md`
  - `lead-qa-synthesizer.md`
- [x] **Verification:** `cargo test persona::builtin` verifies all 12 embedded files parse cleanly.
- [x] **Commit:** `feat(persona): embed 12 Tier-1 core personas into binary`

### Task 4.3: Two-Tier Persona Registry (`src/persona/registry.rs`)
- [x] Implement `PersonaRegistry::load(project_root)`:
  - Loads all 12 built-in Tier-1 personas.
  - Scans `<project_root>/.argus/personas/*.md`.
  - Overrides built-ins if name matches; registers new personas dynamically.
- [x] Squad filtering: `.get_squad("forms")`, `.get_squad("all")`.
- [x] Unit tests for override behavior and dynamic registration.
- [x] **Verification:** `cargo test persona::registry` passes.
- [x] **Commit:** `feat(persona): implement two-tier persona registry with repo overrides`

### Task 4.4: Personas CLI Command (`argus personas`)
- [x] Implement `src/cli/commands/personas.rs`:
  - Subcommands: `list` (table of available personas, squad, source [builtin/local]).
  - `show <name>` (displays full prompt and metadata of a persona).
- [x] **Verification:** `cargo run -- personas list` prints formatted terminal table.
- [x] **Commit:** `feat(cli): add argus personas list and show commands`

---

## Phase 5: Native Pi Subprocess Runner & Concurrency Pool

### Task 5.1: Deterministic Session Naming (`src/runner/session.rs`)
- [x] Implement session name generator: `argus/<branch-slug>/<persona-slug>`.
- [x] Sanitize names for Pi compatibility and filesystem paths.
- [x] Unit tests for session name generation.
- [x] **Verification:** `cargo test runner::session` passes.
- [x] **Commit:** `feat(runner): implement deterministic Pi session naming`

### Task 5.2: Pi Subprocess Launcher (`src/runner/launcher.rs`)
- [x] Implement `launch_agent()`:
  - Invokes `pi` binary with non-interactive mode:
    `pi -p --name <session_name> --append-system-prompt <prompt_file> "<payload>"`
  - Streams stdout and stderr asynchronously.
  - Returns `AgentRunResult` (exit code, output string, duration).
- [x] Handle missing `pi` executable with clean diagnostic error message.
- [x] **Verification:** `cargo test runner::launcher` passes.
- [x] **Commit:** `feat(runner): implement async Pi subprocess launcher`

### Task 5.3: Tokio Concurrency Pool (`src/runner/pool.rs`)
- [x] Implement `SwarmPool`:
  - Manages Tokio `Arc<Semaphore>` for bounded parallelism (default: 4 concurrent agents).
  - Spawns tasks in parallel, collecting results asynchronously.
  - Timeouts per agent session (default: 3 minutes).
- [x] Graceful failure handling: if one agent fails, remaining agents continue.
- [x] Unit tests for pool throttling and timeout handling.
- [x] **Verification:** `cargo test runner::pool` passes.
- [x] **Commit:** `feat(runner): implement Tokio semaphore concurrency pool for swarm execution`

### Task 5.4: Live Multi-Agent Progress UI (`src/runner/progress.rs`)
- [x] Integrate `indicatif::MultiProgress`:
  - Displays concurrent spinner lines for each active persona.
  - Status updates: `running evaluation...` $\to$ `✔ Completed in Xs` / `✖ Failed`.
- [x] Auto-fallback to clean log lines if running in non-TTY / CI environment.
- [x] **Verification:** Visual verification of multi-spinner execution.
- [x] **Commit:** `feat(runner): add live multi-spinner terminal progress UI`

---

## Phase 6: Lead Synthesis & Output Reporting

### Task 6.1: Defect Finding Data Model (`src/synthesis/finding.rs`)
- [ ] Define `Finding` struct:
  - `severity`: `P0Blocker`, `P1Major`, `P2Polish`
  - `title`: concise defect summary
  - `persona`: reporting agent name
  - `file_path`: target file
  - `line_number`: optional line number
  - `oracle_id`: optional matching oracle from `oracles.yaml`
  - `description`: why this breaks
  - `suggested_fix`: concrete code snippet
- [ ] JSON serialization/deserialization for structured output.
- [ ] **Verification:** `cargo test synthesis::finding` passes.
- [ ] **Commit:** `feat(synthesis): define structured defect finding model and severity tiers`

### Task 6.2: Output Aggregator & Lead Synthesizer Orchestrator (`src/synthesis/synthesizer.rs`)
- [ ] Collect raw text reports from all 11 worker sessions.
- [ ] Spawn the `lead-qa-synthesizer` Pi session:
  - Ingests all worker findings.
  - Deduplicates overlapping issues.
  - Emits JSON/structured findings matching `Finding` model.
- [ ] Fallback parser if synthesizer output has formatting quirks.
- [ ] **Verification:** `cargo test synthesis::synthesizer` passes.
- [ ] **Commit:** `feat(synthesis): implement Lead Synthesizer aggregation and deduplication`

### Task 6.3: Terminal Dashboard Renderer (`src/synthesis/terminal.rs`)
- [ ] Render colorized terminal summary:
  - Red `[P0 - BLOCKER]`, Yellow `[P1 - MAJOR]`, Cyan `[P2 - POLISH]`.
  - File paths, lines, and violation descriptions.
  - Total counts and overall audit verdict (`PASSED` / `ACTION REQUIRED`).
- [ ] **Verification:** Test terminal table formatting.
- [ ] **Commit:** `feat(synthesis): implement colorized terminal defect dashboard`

### Task 6.4: Markdown Report Generator (`src/synthesis/markdown.rs`)
- [ ] Generate standalone Markdown report:
  - Executive summary, audit timestamp, git branch, commit SHA.
  - Grouped findings table.
  - Stored at `.argus/reports/YYYY-MM-DD-HHMMSS-audit.md`.
- [ ] **Verification:** `cargo test synthesis::markdown` verifies file generation.
- [ ] **Commit:** `feat(synthesis): implement persistent Markdown audit report generator`

---

## Phase 7: Interactive CLI Commands & End-to-End Verification

### Task 7.1: Interactive Resume Command (`argus resume`)
- [ ] Implement `src/cli/commands/resume.rs`:
  - Discovers active Pi sessions matching `argus/<branch>/*`.
  - Prompts user with interactive selection menu or accepts `--persona <name>`.
  - Executes `pi --resume <session_name>` to drop user into interactive pair-programming.
- [ ] **Verification:** `cargo run -- resume --help` works; session discovery verified.
- [ ] **Commit:** `feat(cli): implement argus resume interactive session picker`

### Task 7.2: End-to-End `argus audit` Command (`src/cli/commands/audit.rs`)
- [ ] Wire all components into unified pipeline:
  `argus audit [--squad <name>] [--sfd <path>] [--base <branch>] [--staged]`
- [ ] Steps: Git Diff $\to$ Noise Filter $\to$ Context Bundle $\to$ Persona Registry $\to$ Concurrency Pool $\to$ Synthesis $\to$ Terminal & Markdown Report.
- [ ] Exit codes: `0` (clean / only P2), `1` (P0 or P1 defects found).
- [ ] **Verification:** `cargo test` full test suite passes.
- [ ] **Commit:** `feat(cli): wire end-to-end argus audit workflow`

### Task 7.3: Field Verification on Work Codebase (`superapps`)
- [ ] Run `argus audit` on `superapps` branch `DEV/AFM/AUTH_MIGRATION`.
- [ ] Verify that Argus flags:
  - RT/RW textfield without length limiting.
  - NPWP Coretax format requirements.
  - Unpushed invoice deletion without stock restoration.
- [ ] Document verification results in `docs/verification-report.md`.
- [ ] **Commit:** `docs: record initial field verification results on superapps codebase`
