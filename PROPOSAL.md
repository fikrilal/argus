# Engineering Proposal: Argus — Autonomous Pre-Flight QA Swarm

**Status:** Approved (Implementation Phase)  
**Target Repository:** `fikrilal/argus`  
**Author:** ahmad fikril (<fikrildev@gmail.com>)  
**Version:** 1.0.0  
**Date:** October 2026  
**Related Discussion Records:** [`discussions/01` through `12`](./discussions/)  

---

## 1. Executive Summary

We propose and specify **Argus**, a high-performance, universal CLI orchestrator written in **Rust** that deploys an **autonomous, multi-agent adversarial QA swarm** against code changes prior to human QA handoff.

Argus transforms the pre-handoff engineering workflow. Instead of relying on single-path "golden" unit tests that only verify what the developer intended to build, Argus deploys **12 specialized adversarial agent personas** running in parallel native **Pi.dev** sessions. These agents cross-reference the System Functional Design (SFD), audit state and ledger reversibility, enforce regional regulations, and fuzz input boundaries.

### Core Objectives:
1. **Eliminate Developer Knowledge Asymmetry:** Catch what the engineer *"forgot"*, *"was not aware of"*, or *"didn't know"* by cross-referencing converted Markdown SFD specifications.
2. **Enforce Non-Golden Testing:** Move beyond happy-path tests to property-based verification, state reversibility laws ($\Delta \text{State} = 0$), and offline resilience.
3. **Resumable Interactive Remediation:** Every agent runs in a persistent, named Pi session (`pi -r argus/<branch>/<persona>`), allowing the engineer to seamlessly pair-program on fixes.
4. **Universal & Zero-Friction:** Built in Rust as a single standalone native binary with sub-3ms startup, usable across mobile (Flutter, iOS, Android), frontend, and backend repositories.

---

## 2. Problem Statement & Root Cause Analysis

### 2.1 The Field Experience (Kalbe Nutritionals Super App)
During the delivery of the offline-first enterprise distribution module (**`simplidot`** in `falcon_kalbe_app`), features that passed developer unit tests and manual sanity checks encountered repeated defect reports from human QA. 

Typical defect patterns included:
- **Input Boundaries:** In `add_customer_page.dart`, `RT` and `RW` text fields accepted arbitrary character strings and unconstrained lengths because `AppTextFieldType.number` only set soft keyboard hints without text formatters.
- **Regulatory Nuances:** NPWP validation rejected formatted or 16-digit corporate IDs under Indonesian DJP Coretax because the validator was hardcoded to personal 15-digit numbers without punctuation.
- **Ledger Invariant Leaks:** Deleting an unpushed, pending sales invoice from the upload queue deleted the SQLite outbox row without restoring the deducted motoris inventory or cleaning up staged photo receipts.
- **Defensive Resilience Gaps:** Operational download mappers threw `FormatException` on HTTP 200 with `data: null` (affecting new sales reps with empty history), and sync executors threw `StateError` if Android OS storage cleanup deleted a cached photo.
- **Identity Mismatches:** Code compared SuperApp IAM GUIDs with domain ERP GUIDs, failing sync despite employee NIKs being identical.

### 2.2 Root Cause: Knowledge Asymmetry & The Builder Bias
These defects were not caused by poor coding skills. They were caused by **Knowledge Asymmetry** and cognitive fatigue:

```
┌───────────────────────────────────────┐       ┌───────────────────────────────────────┐
│           DEVELOPER MINDSET           │       │              QA MINDSET               │
│            ("BUILDER MODE")           │       │           ("BREAKER MODE")            │
├───────────────────────────────────────┤       ├───────────────────────────────────────┤
│ • Focus: Does the feature work?       │  VS   │ • Focus: How can I break this?        │
│ • Tests verify intended functionality │       │ • Tests attack unintended boundaries  │
│ • Assumes cooperative user inputs     │       │ • Assumes hostile / chaotic inputs    │
│ • Verifies forward progress (A -> B)  │       │ • Verifies reversal (A -> B -> Undo)  │
└───────────────────────────────────────┘       └───────────────────────────────────────┘
```

When building a feature, developers write tests to confirm that their implementation works. But human QA tests what happens when things go wrong:
- *"I Forgot":* Skimming a 45-page SFD specification causes cognitive fatigue, resulting in overlooked sub-clauses.
- *"I Was Not Aware":* Building the checkout flow without knowing that offline drafts require an exact, symmetrical inventory return mechanism.
- *"I Didn't Know":* Lacking historical knowledge of regional regulatory standards (Coretax, Kemendagri RT/RW padding).

---

## 3. Testing Philosophy: The Three Non-Golden Pillars

To eliminate defects before human QA, Argus is built upon three non-golden testing pillars:

```
┌────────────────────────────────────────────────────────────────────────┐
│                      NON-GOLDEN TESTING METHODOLOGY                    │
├─────────────────────┬──────────────────────┬───────────────────────────┤
│     PILLAR 1:       │      PILLAR 2:       │         PILLAR 3:         │
│   PROPERTY-BASED    │    METAMORPHIC &     │       STATE MACHINE       │
│      TESTING        │   INVARIANT LAWS     │          CHAOS            │
├─────────────────────┼──────────────────────┼───────────────────────────┤
│ Feed 1,000+ pseudo- │ Assert algebraic     │ Execute non-linear,       │
│ random, adversarial │ conservation laws    │ shuffled action sequences │
│ inputs to discover  │ across state         │ to expose race conditions │
│ unhandled crashes.  │ transitions.         │ and re-entrancy bugs.     │
└─────────────────────┴──────────────────────┴───────────────────────────┘
```

1. **Property-Based Testing:** Enforcing the *Crash-Proof Property* ($\forall s \in \text{String}: \text{throw}(\text{UnhandledException}) = \text{False}$). Validators and mappers must never throw unhandled exceptions on dirty, oversized, or malformed inputs.
2. **The Reversibility Law ($\Delta \text{State} = 0$):** Any local offline transaction must be strictly reversible:
   $$\text{State}_0 \xrightarrow{+\text{Create}(\text{Entity})} \text{State}_1 \xrightarrow{+\text{Delete}(\text{Entity})} \text{State}_2 \implies \text{State}_2 \equiv \text{State}_0$$
3. **State Machine Chaos:** Verifying re-entrancy (double-tapping submit), mid-flight cancellation during asynchronous database transactions, and graceful recovery from process termination.

---

## 4. The Argus Swarm Architecture

Argus dispatches **12 specialized agent personas** organized into **4 functional squads**, coordinated by a **Lead Synthesizer**:

```text
┌────────────────────────────────────────────────────────────────────────┐
│                      THE 12-PERSONA ARGUS SWARM                        │
├────────────────────────────────────────────────────────────────────────┤
│ SQUAD 1: SPEC & DOMAIN AUTHORITY (Solves "Forgot" & "Didn't Know")      │
│  1. sfd-clause-detective      : Cross-references converted SFD clauses │
│  2. id-regulatory-sentinel    : Enforces NPWP Coretax, NIK, RT/RW rules│
│  3. rbac-identity-gatekeeper  : Audits roles, IAM vs ERP GUIDs, multi-app│
├────────────────────────────────────────────────────────────────────────┤
│ SQUAD 2: INPUT & INTERACTION STRESS (The Hostile User)                 │
│  4. form-boundary-saboteur    : Injects dirty strings, length attacks  │
│  5. cascade-dropdown-glitcher : Verifies child resets on parent change │
│  6. concurrency-double-tapper : Audits debounce, re-entrancy, back nav │
├────────────────────────────────────────────────────────────────────────┤
│ SQUAD 3: DATA, LEDGER & PERSISTENCE (The Mathematical Auditor)         │
│  7. stock-ledger-auditor      : Enforces Reversibility Law & rollbacks │
│  8. orphan-cascade-hunter     : Detects ghost child rows & foreign keys│
├────────────────────────────────────────────────────────────────────────┤
│ SQUAD 4: ENVIRONMENT, OFFLINE & CHAOS (Murphy's Law in the Field)      │
│  9. payload-pessimist         : Audits HTTP 200 nulls, empty fallbacks │
│ 10. sync-deadlock-guard       : Prevents outbox stalls on missing media│
│ 11. hardware-sensor-adversary : Audits GPS jitter, low storage, perms  │
├────────────────────────────────────────────────────────────────────────┤
│ COMMAND CORE                                                           │
│ 12. lead-qa-synthesizer       : Triages findings, P0-P2, generates tests│
└────────────────────────────────────────────────────────────────────────┘
```

---

## 5. Technology Stack & Rust Architecture

### 5.1 Why Rust?
- **Zero Runtime Dependencies:** Compiles to a single, statically linked native binary (`argus`). Developers do not need Node.js, Dart, Go, or Python installed to run Argus.
- **Performance & Startup:** Sub-3ms startup time and $< 10\text{MB}$ memory footprint, avoiding JVM/Node runtime bloat.
- **Concurrency & Process Safety:** Tokio’s async runtime allows spawning and monitoring 12 concurrent sub-processes with zero thread leakage.
- **Rich Terminal UI:** `indicatif` provides concurrent spinners for each active agent, and `colored` delivers clean terminal reporting.

### 5.2 Package Architecture (`fikrilal/argus`)

```text
src/
├── main.rs                      # Binary entry point & CLI dispatch
├── cli/                         # Command-line interface definitions (clap)
│   ├── args.rs                  # CLI flags & subcommand parsers
│   └── commands/                # Subcommand handlers (audit, init, personas, resume)
├── config/                      # .argus/config.yaml parser & models (serde)
├── git/                         # Git diff extraction & noise stripping
├── context/                     # SFD & specification ingestion
├── persona/                     # Two-tier persona registry & discovery
├── runner/                      # Native Pi subprocess launcher & pool (tokio)
├── synthesis/                   # Finding aggregator, ranker & report generators
└── oracles/                     # .argus/oracles.yaml parser & validator
```

---

## 6. Native Pi Harness Integration & Resumable Sessions

Argus deliberately rejects ephemeral, toy subagent extensions in favor of **native, first-class Pi.dev sessions**:

### 6.1 Deterministic Session Naming Scheme
Sessions are deterministically named using a hierarchical pattern:
```text
argus/<branch-slug>/<persona-slug>
```
*Example on branch `DEV/AFM/AUTH_MIGRATION`:*
- `argus/DEV-AFM-AUTH-MIGRATION/sfd-clause-detective`
- `argus/DEV-AFM-AUTH-MIGRATION/stock-ledger-auditor`
- `argus/DEV-AFM-AUTH-MIGRATION/id-regulatory-sentinel`

### 6.2 Resumable Pair-Programming (`pi -r`)
When Argus flags an issue (e.g. `stock-ledger-auditor` detects an unpushed faktur deletion without inventory restoration), the developer can immediately jump into that exact session:
```bash
pi -r argus/DEV-AFM-AUTH-MIGRATION/stock-ledger-auditor
```
Because the session is persistent, the agent retains the full git diff, file context, and analysis, allowing the developer to say:
> *"How do I implement this atomic stock rollback in Drift SQLite?"*
The agent writes the exact transaction code, closing the loop immediately.

---

## 7. Repository Standardization: The `.argus/` Specification

Every repository adopting Argus defines its configuration and specialized rules inside a local `.argus/` directory:

```text
<repository-root>/
├── .argus/
│   ├── config.yaml               # Active squads, model overrides, SFD location
│   ├── oracles.yaml              # Machine-checked business invariants
│   ├── personas/                 # Project-specific persona overrides & additions
│   └── context/
│       └── sfd/
│           └── active_spec.md    # Converted Markdown SFD
```

### Two-Tier Inheritance
1. **Tier 1 (Built-In Core):** Shipped directly within the compiled `argus` binary (generic form saboteur, double-tapper, payload pessimist, orphan hunter).
2. **Tier 2 (Project-Local):** Extends or overrides base personas with project-specific domain rules (e.g. Kalbe's `id-regulatory-sentinel` or `stock-ledger-auditor`).

---

## 8. Implementation Roadmap

| Phase | Milestone | Deliverables |
|---|---|---|
| **Phase 1** | **Scaffolding & Core CLI** | Rust package setup (`Cargo.toml`), `clap` CLI commands (`audit`, `init`, `personas`, `resume`). |
| **Phase 2** | **Git & Context Engine** | `git::diff` with noise filtering (ignoring `*.g.dart`, locks, build artifacts); SFD Markdown reader. |
| **Phase 3** | **Persona Registry & Runner** | Two-tier persona resolver (embedded + `.argus/personas/`); Tokio async `pi` process launcher with concurrency limits. |
| **Phase 4** | **Synthesis & Terminal UI** | Lead Synthesizer aggregation, `indicatif` multi-agent spinners, P0/P1/P2 defect ranking, and Markdown report output. |
| **Phase 5** | **Field Validation** | Run Argus on `superapps` branch `DEV/AFM/AUTH_MIGRATION` to verify real-world detection of known QA bugs. |

---

## 9. Expected Impact & Success Metrics

1. **Defect Density Reduction:** Target $> 80\%$ reduction in defects logged by human QA per sprint.
2. **Cycle Time Compression:** Eliminate 2–3 rework loops between Developer and QA, enabling single-pass sign-offs.
3. **Developer KPI Protection:** Trivial boundary flaws and forgotten spec sub-clauses are resolved before code review, protecting developer quality ratings.
4. **Institutional Knowledge Retention:** Every bug caught by QA in the future is added to `.argus/personas/`, transforming Argus into a permanent, immortal institutional memory.
