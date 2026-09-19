<div align="center">

# 👁️ ARGUS

**The All-Seeing Pre-Flight QA Swarm for Modern Engineering**

*Autonomous multi-agent adversarial stress-testing. Built with Rust.*

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Built with Rust](https://img.shields.io/badge/Built%20with-Rust-orange.svg)](https://www.rust-lang.org/)
[![Orchestrator: Pi](https://img.shields.io/badge/Orchestrator-Pi.dev-purple.svg)](https://pi.dev)

</div>

---

## Why Argus?

> *In Greek mythology, Argus Panoptes was the giant of a hundred eyes—vigilant, untiring, and all-seeing. When some eyes slept, the others kept watch.*

Human developers have two eyes and experience cognitive fatigue. When building features along the golden path, subtle state invariants, forgotten SFD sub-clauses, and regional formatting rules slip past into QA reports, degrading engineering KPIs.

**Argus unleashes a 12-agent adversarial swarm on your git diff**—cross-referencing specification documents, auditing state reversibility, checking regional regulations, and fuzzing inputs before human QA ever sees the code.

---

## Architectural Foundations & Design Records

The core research, discussions, and design specifications are cataloged in [`discussions/`](./discussions/):

1. **[`01-problem-statement-and-context.md`](./discussions/01-problem-statement-and-context.md)**  
   *Problem Statement, Context & Objectives* — Analysis of why developer unit testing misses QA bugs, impact on KPI, and the core challenge we are solving.

2. **[`02-non-golden-testing-paradigms.md`](./discussions/02-non-golden-testing-paradigms.md)**  
   *Beyond Happy Paths: Non-Golden Testing Paradigms* — Property-based testing, invariant/metamorphic testing, and state machine fuzzing.

3. **[`03-qa-swarm-architecture-and-agent-personas.md`](./discussions/03-qa-swarm-architecture-and-agent-personas.md)**  
   *Universal QA Swarm Architecture & Agent Personas* — Swarm workflow and deep-dive into specialized adversarial personas.

4. **[`04-kalbe-domain-playbook-and-edge-cases.md`](./discussions/04-kalbe-domain-playbook-and-edge-cases.md)**  
   *Domain-Specific Test Matrix & Playbook* — Concrete catalog of Indonesian localization patterns, Drift/SQLite offline invariants, and resilience rules.

5. **[`05-implementation-roadmap-and-next-steps.md`](./discussions/05-implementation-roadmap-and-next-steps.md)**  
   *Implementation Roadmap & Tooling Strategy* — Step-by-step roadmap from CLI prototype to automated non-golden test generation.

6. **[`06-qa-swarm-mechanics-and-pi-harness-integration.md`](./discussions/06-qa-swarm-mechanics-and-pi-harness-integration.md)**  
   *QA Swarm Mechanics & Pi Harness Integration* — Deep dive into multi-agent subagent mechanics, why swarms outperform single agents, and native Pi integration.

7. **[`07-knowledge-asymmetry-sfd-and-resumable-pi-sessions.md`](./discussions/07-knowledge-asymmetry-sfd-and-resumable-pi-sessions.md)**  
   *Knowledge Asymmetry, SFD Ground Truth & Resumable Pi Sessions* — Rejection of toy subagents in favor of native resumable `pi -r` sessions, SFD (System Functional Design) Markdown ingestion, and the 4 Knowledge Pillars solving "Forgot", "Not Aware", and "Don't Know".

8. **[`08-harness-engineering-inspiration-mobile-core-kit.md`](./discussions/08-harness-engineering-inspiration-mobile-core-kit.md)**  
   *Harness Engineering Inspiration: Mobile Core Kit & Custom Swarm CLI* — Architectural lessons from `mobile-core-kit` (typed CLI, task authority, behavioral oracles, parallel isolation) and how to design the custom Swarm CLI orchestrator for native Pi sessions.

9. **[`09-incubation-strategy-mobile-core-kit.md`](./discussions/09-incubation-strategy-mobile-core-kit.md)**  
   *Incubation Strategy: Sandbox in Mobile Core Kit Before Work Rollout* — Strategic decision to incubate, test, and stabilize the QA Swarm before graduating it into the production work repository.

10. **[`10-repository-standardization-and-dot-swarm-spec.md`](./discussions/10-repository-standardization-and-dot-swarm-spec.md)**  
    *Repository Standardization: The `.swarm/` Specification & Dynamic Personas* — Standardizing repository-local `.swarm/` layout, two-tier persona inheritance (core built-ins vs repo overrides), `.swarm/config.yaml` schema, and persona prompt formats.

11. **[`11-standalone-cli-architecture-and-implementation-plan.md`](./discussions/11-standalone-cli-architecture-and-implementation-plan.md)**  
    *Standalone CLI Architecture & Implementation Plan* — Rationale for standalone Rust CLI package (`argus`), directory layout, core implementation modules, dependencies, and development phases.

12. **[`12-argus-end-to-end-system-architecture.md`](./discussions/12-argus-end-to-end-system-architecture.md)**  
    *Argus End-to-End System Architecture* — Detailed three-tier topology, 8-stage data pipeline, Rust module contracts (`src/`), Tokio semaphore concurrency model, and failure mapping.

---

## License

MIT © [Ahmad Fikri](https://github.com/fikrilal)
