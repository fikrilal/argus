# 05 — Implementation Roadmap & Tooling Strategy

**Document Status:** Approved Discussion Record  
**Target Domain:** Tooling Implementation, Integration & Rollout  
**Date:** September 2026  

---

## 1. Roadmap Overview

To transform the QA Swarm from an architectural concept into an everyday developer utility, the implementation is structured across four progressive phases:

```
┌────────────────────────────────────────────────────────────────────────┐
│                   QA SWARM IMPLEMENTATION ROADMAP                      │
├──────────────────┬──────────────────┬──────────────────┬───────────────┤
│     PHASE 1      │     PHASE 2      │     PHASE 3      │    PHASE 4    │
│  Persona Specs   │  Pilot Audit &   │ CLI Orchestrator │  Non-Golden   │
│  & Prompts       │  Calibration     │ & Git Workflow   │ Test Generator│
├──────────────────┼──────────────────┼──────────────────┼───────────────┤
│ Codify the 5     │ Run swarm audit  │ Build standalone │ Auto-generate │
│ agent personas   │ against recent   │ Dart/Bash runner │ runnable Dart │
│ in markdown      │ Git diffs in     │ for local        │ regression    │
│ and system rules.│ this repository. │ pre-flight use.  │ tests.        │
└──────────────────┴──────────────────┴──────────────────┴───────────────┘
```

---

## 2. Phase Breakdown

### Phase 1: Persona Specification & Playbook Codification
- Formally write the prompt templates and inspection rubrics for the 5 agents:
  - `agent-1-input-saboteur.md`
  - `agent-2-invariant-hunter.md`
  - `agent-3-network-pessimist.md`
  - `agent-4-chaos-abuser.md`
  - `agent-5-lead-synthesizer.md`
- Include the **Kalbe Domain Playbook** as an injectable module for domain-specific checks.

### Phase 2: Pilot Run & Baseline Calibration
- Pilot the personas against the current branch (`DEV/AFM/AUTH_MIGRATION`) and unstaged changes:
  - `NpwpValidator` modifications.
  - `AddCustomerPage` RT/RW input formatting.
  - Unpushed invoice deletion and stock mutation handling.
- Verify whether the Swarm flags the exact issues that previously tripped up QA handoffs.
- Tune false-positive rates to ensure recommendations are high-signal.

### Phase 3: CLI Orchestrator & Git Integration
- Develop a lightweight runner (e.g. `tool/qa_swarm/run_qa_swarm.sh` or Dart CLI `bin/qa_swarm.dart`):
  - Automatically captures `git diff` against a target branch (`origin/main` or base branch).
  - Identifies affected layers: UI widgets, GetX controllers, Drift DAOs, Retrofit API clients.
  - Routes the diff through the multi-agent review pipeline.
  - Emits a clean terminal or Markdown report with prioritized action items.

### Phase 4: Autonomous Non-Golden Test Synthesis
- Enable Agent 5 (Lead QA Synthesizer) to emit executable Dart test code:
  - Generates `test/adversarial/*_adversarial_test.dart`.
  - Implements property-based fuzzing and state machine reversals using the project's existing test helpers (`wrap()`, mock DAOs, in-memory Drift databases).
  - Allows the engineer to run `flutter test` immediately to verify fixes.

---

## 3. Success Metrics & Performance Impact

| Metric | Baseline (Pre-Swarm) | Target (Post-Swarm) |
|---|---|---|
| **Defects Caught by QA** | 5–10 minor/edge bugs per feature handoff | $< 1$ defect per feature handoff |
| **QA Turnaround Cycles** | 2–3 rework loops per sprint | 1 single-pass sign-off |
| **Engineer KPI Standing** | Vulnerable to "silly" bug logging | High delivery velocity with verified quality |
| **Test Suite Quality** | 95% happy-path golden tests | Balanced coverage including boundary and reversal tests |
