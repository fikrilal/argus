# 09 — Incubation Strategy: Sandbox in Mobile Core Kit Before Work Rollout

**Document Status:** Approved Discussion Record  
**Target Domain:** Tooling Staging, Environment Isolation & Sandbox Strategy  
**Date:** September 2026  

---

## 1. The Strategy: Two-Stage Incubation & Rollout

A critical strategic decision was established: the **QA Swarm system will NOT be built directly inside the active work repository (`falcon_kalbe_app` / `superapps`)**.

Instead, the system follows a clean **two-stage incubation lifecycle**:

```
┌────────────────────────────────────────────────────────────────────────┐
│                   TWO-STAGE INCUBATION & ROLLOUT                       │
├───────────────────────────────────┬────────────────────────────────────┤
│             STAGE 1:              │              STAGE 2:              │
│       INCUBATION & EXPERIMENT     │        PRODUCTION GRADUATION       │
│        (`mobile-core-kit`)        │        (`superapps` / Work)        │
├───────────────────────────────────┼────────────────────────────────────┤
│ • Full flexibility and freedom    │ • Battle-tested and stable         │
│ • Existing god-tier harness & CLI │ • Zero experimentation overhead    │
│ • Rich mock fixtures & oracles    │ • Immediate KPI protection         │
│ • Zero pollution of work git tree │ • Proven edge-case detection       │
└───────────────────────────────────┴────────────────────────────────────┘
```

---

## 2. Why `mobile-core-kit` is the Ideal Incubation Sandbox

`/home/fikrilal/workspace/devs/core/mobile-core-kit` provides an unmatched foundation for developing the Swarm CLI and agent orchestration:

1. **Pre-Built CLI Infrastructure:**
   - Already has `packages/mobile_core_kit_cli` with argument parsing (`package:args`), typed workflow handlers, command runners, and platform abstractions.
   - The swarm can be developed either as a dedicated command suite within `mobilekit` (e.g. `mobilekit qa swarm`) or as an independent companion package (`packages/qa_swarm_cli`).
2. **Deterministic Task Authority & Safety:**
   - The workspace enforces strict file scoping, preflight checks, and immutable execution plans, ensuring agents cannot mutate unintended files during automated test runs.
3. **Behavioral Oracles Registry (`harness/oracles.yaml`):**
   - The oracle architecture is already implemented and validated, making it trivial to plug in QA and domain-invariant oracles.
4. **Freedom to Iterate Without Corporate Pressure:**
   - In `mobile-core-kit`, experimental features can be fuzzed, refactored, and benchmarked without worrying about sprint velocity or active QA scrutiny.

---

## 3. Graduation & Cutover Plan to Work (`superapps`)

Once the QA Swarm engine achieves stability in `mobile-core-kit`:

1. **Porting the Artifacts:**
   - The finalized CLI executable / scripts and persona prompt Markdown definitions (`personas/*.md`) can be cleanly transferred or linked.
   - The **Kalbe Domain Playbook** (`04-kalbe-domain-playbook-and-edge-cases.md`) and the project's converted Markdown SFD are plugged in as domain context.
2. **First Work Mission:**
   - Running the graduated Swarm against active branches (such as `DEV/AFM/AUTH_MIGRATION`) before opening formal pull requests or handing off to human QA.
3. **KPI Impact:**
   - The work codebase receives an already mature, hardened tool that immediately suppresses defect density without any disruptive "building in flight" friction.
