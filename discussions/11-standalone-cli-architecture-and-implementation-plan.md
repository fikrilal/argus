# 11 — Standalone CLI Architecture & Implementation Plan

**Document Status:** Approved Discussion Record  
**Target Domain:** Standalone Dart CLI Package Design & Component Implementation Plan  
**Date:** October 2026  

---

## 1. Decision: Standalone CLI over Monorepo Bundling

A key architectural decision was finalized: **`qaswarm` will be developed as an independent, standalone Dart CLI package**, completely decoupled from `mobile-core-kit` or any specific application repository.

### Rationale:
1. **Zero Coupling & True Portability:** Can be run against any Flutter, React Native, iOS, Android, or backend codebase without pulling in unrelated monorepo dependencies.
2. **Global Activation via Dart Pub:** Can be activated globally on the developer machine:
   ```bash
   dart pub global activate --source path /home/fikrilal/workspace/devs/experiments/qa-swarm
   ```
   This allows running `qaswarm audit` in *any* directory or project terminal.
3. **Independent Release & Versioning:** Evolutionary improvements, new personas, and bugfixes can be released without synchronizing with app sprint release cycles.
4. **Single Responsibility:** The CLI exists for one singular purpose: orchestrating adversarial pre-flight QA swarms via native Pi sessions.

---

## 2. Package Identity & Structure

- **Package Name:** `qa_swarm`
- **CLI Executable:** `qaswarm`
- **Location:** `/home/fikrilal/workspace/devs/experiments/qa-swarm`

### Directory Layout:

```text
qa-swarm/
├── bin/
│   └── qaswarm.dart                 # CLI entry point
├── lib/
│   ├── qa_swarm.dart                # Library export
│   └── src/
│       ├── cli/                     # CLI argument handling (package:args)
│       │   ├── qaswarm_command_runner.dart
│       │   └── commands/
│       │       ├── audit_command.dart
│       │       ├── init_command.dart
│       │       ├── list_command.dart
│       │       └── resume_command.dart
│       ├── config/                  # .swarm/config.yaml parser
│       │   ├── swarm_config.dart
│       │   └── squad_definition.dart
│       ├── git/                     # Git operations & diff filtering
│       │   ├── git_diff_provider.dart
│       │   └── branch_detector.dart
│       ├── context/                 # SFD and domain context intake
│       │   ├── sfd_provider.dart
│       │   └── context_bundle.dart
│       ├── personas/                # Two-tier persona registry
│       │   ├── persona_definition.dart
│       │   ├── persona_registry.dart
│       │   └── built_in_personas.dart # Embedded base personas
│       ├── runner/                  # Native Pi session spawner
│       │   ├── pi_process_launcher.dart
│       │   ├── pi_session_runner.dart
│       │   └── session_naming.dart
│       ├── synthesis/               # Aggregation & report formatting
│       │   ├── finding.dart
│       │   └── report_formatter.dart
│       └── oracles/                 # .swarm/oracles.yaml parser
│           └── oracle_registry.dart
├── test/                            # Comprehensive unit tests
├── pubspec.yaml
└── discussions/                     # Architectural design records
```

---

## 3. Core Implementation Modules

### Module 1: `git_diff_provider.dart`
- Captures `git diff` against base branches (`origin/main`, `origin/development`, or working tree).
- **Noise Filter:** Automatically strips out generated files (`*.g.dart`, `*.freezed.dart`, `pubspec.lock`, `*.lock`, build artifacts) to maximize LLM reasoning bandwidth on human-written logic.

### Module 2: `persona_registry.dart`
- Discovers and loads personas with **Two-Tier Inheritance**:
  - Embedded Tier-1 base personas (built into the Dart binary).
  - Project-local Tier-2 personas from `<repo>/.swarm/personas/*.md`.
- Parses frontmatter (`name`, `title`, `squad`, `model_tier`, `tools`).

### Module 3: `pi_process_launcher.dart`
- Launches native `pi` subprocesses via Dart's `Process.start`.
- Enforces deterministic session naming: `qa-swarm/<branch-slug>/<persona-slug>`.
- Controls concurrency (e.g. running 4 worker sessions in parallel).
- Non-interactive execution (`-p`) that streams progress while persisting session state for subsequent `pi -r` resumption.

### Module 4: `sfd_provider.dart`
- Ingests converted Markdown SFD specifications.
- Bundles the SFD with the git diff and injects it into personas that declare SFD context requirements (e.g. `sfd-clause-detective`).

### Module 5: `report_formatter.dart`
- Aggregates findings from worker sessions.
- Formats terminal output with visual severity tags:
  - `[BLOCKER / P0]`: Data loss, stock ledger imbalance, unhandled crashes on HTTP 200 OK.
  - `[MAJOR / P1]`: Unsanitized form inputs, bypassed business validations, cascade dropdown errors.
  - `[POLISH / P2]`: Minor formatting, missing soft limits, cosmetic feedback.
- Writes a Markdown summary to `.swarm/reports/<timestamp>-audit.md`.

---

## 4. Dependencies (`pubspec.yaml`)

```yaml
name: qa_swarm
description: A universal adversarial pre-flight QA swarm orchestrator powered by native Pi sessions.
version: 0.1.0
publish_to: none

environment:
  sdk: ^3.7.0

dependencies:
  args: ^2.5.0
  yaml: ^3.1.2
  path: ^1.9.0
  meta: ^1.15.0

dev_dependencies:
  test: ^1.25.0
  lints: ^5.0.0
```

---

## 5. Development Phases

1. **Step 1: Scaffolding Dart Package:** Set up `pubspec.yaml`, `bin/qaswarm.dart`, and base command runner.
2. **Step 2: Git & Persona Engine:** Implement `GitDiffProvider`, noise filtering, and `PersonaRegistry` with frontmatter parsing.
3. **Step 3: Pi Runner Engine:** Implement `PiProcessLauncher` and session naming logic.
4. **Step 4: Report Synthesis:** Build output aggregator and terminal/markdown reporting.
5. **Step 5: Testing & Global Activation:** Verify with unit tests, activate globally, and test against `superapps`.
