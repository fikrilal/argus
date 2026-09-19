# 08 — Harness Engineering Inspiration: Mobile Core Kit & Custom Swarm CLI

**Document Status:** Approved Discussion Record  
**Target Domain:** CLI Architecture, Session Orchestration & Lessons from Mobile Core Kit  
**Date:** September 2026  

---

## 1. Exploring `mobile-core-kit`: A Benchmark for Harness Engineering

A deep dive into `/home/fikrilal/workspace/devs/core/mobile-core-kit` reveals a world-class standard for developer and agent harness engineering:

### Key Architectural Pillars in `mobile-core-kit`:
1. **Dedicated Dart CLI Package (`packages/mobile_core_kit_cli`):**
   - Built using standard `package:args/args.dart`.
   - Strongly-typed commands, structured workflows (`VerifyWorkflow`, `LintWorkflow`, `TaskWorkflow`), and platform-aware process execution (`CommandRunner`).
2. **Task Authority & Evidence-Based Verification (`task_authority.md`):**
   - Work is governed by active execution plans declaring file scopes, allowed actions (`edit`, `verify`, `commit`), and risk ceilings.
   - Tasks cannot claim completion without machine evidence (test fingerprints, logcat signals, diff hashes).
3. **Behavioral Oracles (`harness/oracles.yaml`):**
   - A machine-readable registry linking high-risk impacts (auth, session persistence, contracts, navigation) to concrete verification targets (integration tests, Maestro UI flows, regression tests).
4. **Parallel Agent Workflow (`parallel_agent_workflow.md`):**
   - Strictly enforces workspace isolation (`git worktree` per agent) to prevent cross-agent staging mistakes, noise, and dirty diff pollution.

---

## 2. Designing the Custom QA Swarm CLI

Drawing direct inspiration from `mobile-core-kit`, we can design a dedicated Dart CLI package: **`qa_swarm_cli`**.

The CLI serves as the **master orchestrator** that bridges:
1. Local Git workspace state (`git diff`, modified layers).
2. The converted SFD Markdown specification.
3. Native, persistent Pi agent sessions.

---

## 3. Session Naming & Deterministic Identity Scheme

To make sessions intuitive and cleanly discoverable via `pi -r`, the CLI adopts a deterministic, hierarchical naming convention:

```text
qa-swarm/<branch-slug>/<persona-slug>
```

### Concrete Example on Branch `DEV/AFM/AUTH_MIGRATION`:
- `qa-swarm/DEV-AFM-AUTH-MIGRATION/sfd-analyst`
- `qa-swarm/DEV-AFM-AUTH-MIGRATION/id-compliance`
- `qa-swarm/DEV-AFM-AUTH-MIGRATION/state-architect`
- `qa-swarm/DEV-AFM-AUTH-MIGRATION/chaos-tester`
- `qa-swarm/DEV-AFM-AUTH-MIGRATION/lead-synthesizer`

### Why This is Game-Changing in Daily Use:
When the developer runs `pi -r` (or `pi --resume`) in their terminal, Pi displays a clean, organized session menu:
```text
Choose session to resume:
> qa-swarm/DEV-AFM-AUTH-MIGRATION/sfd-analyst       (5m ago)
  qa-swarm/DEV-AFM-AUTH-MIGRATION/state-architect   (8m ago)
  qa-swarm/DEV-AFM-AUTH-MIGRATION/id-compliance     (12m ago)
  qa-swarm/DEV-AFM-AUTH-MIGRATION/chaos-tester      (15m ago)
  qa-swarm/DEV-AFM-AUTH-MIGRATION/lead-synthesizer  (2m ago)
```
The developer can immediately jump into any agent to continue the dialogue or request an implementation fix.

---

## 4. CLI Execution Lifecycle & Process Spawning

```text
Developer runs:
$ qaswarm audit --diff --sfd docs/sfd/stockist_kulakan.md
                            │
                            ▼
┌────────────────────────────────────────────────────────┐
│               STEP 1: CONTEXT INTAKE                   │
│ • Captures `git diff` against base branch              │
│ • Reads converted Markdown SFD                         │
│ • Detects affected layers (UI, DAOs, Controllers)      │
└───────────────────────────┬────────────────────────────┘
                            │
                            ▼
┌────────────────────────────────────────────────────────┐
│          STEP 2: PARALLEL AGENT DISPATCH               │
│ Spawns 4 concurrent `pi` subprocesses via `Process.start`:│
│                                                        │
│ pi -p --name "qa-swarm/$BRANCH/sfd-analyst" \         │
│       --append-system-prompt @personas/sfd.md "..."    │
│                                                        │
│ pi -p --name "qa-swarm/$BRANCH/state-architect" \      │
│       --append-system-prompt @personas/state.md "..."  │
│                                                        │
│ (All agents run non-interactively, saving session data)│
└───────────────────────────┬────────────────────────────┘
                            │
                            ▼
┌────────────────────────────────────────────────────────┐
│               STEP 3: LEAD SYNTHESIS                   │
│ Pipes all 4 outputs into the Synthesizer session:      │
│ pi -p --name "qa-swarm/$BRANCH/lead-synthesizer" ...   │
│ Produces Pre-Flight QA Defect Report & Test Stubs      │
└───────────────────────────┬────────────────────────────┘
                            │
                            ▼
┌────────────────────────────────────────────────────────┐
│             STEP 4: INTERACTIVE RESUME                 │
│ Found an invariant bug? Open that agent immediately:   │
│ $ pi -r qa-swarm/DEV-AFM-AUTH-MIGRATION/state-architect│
└────────────────────────────────────────────────────────┘
```

---

## 5. Integrating the "Oracle" Concept for Quality Assurance

Borrowing `mobile-core-kit`'s behavioral oracle registry (`harness/oracles.yaml`), the QA Swarm can define **Domain Invariant Oracles**:

```yaml
schemaVersion: 1
oracles:
  stock.reversibility:
    kind: state-invariant
    rule: "Deleting an unpushed outbox transaction must emit a compensatory stock mutation"
    covers: [simplidot, inventory, sales]
    
  regional.id.boundaries:
    kind: validation-contract
    rule: "RT/RW fields must enforce digitsOnly and length <= 3"
    covers: [simplidot, customer, forms]
    
  tax.npwp.coretax:
    kind: regulatory-compliance
    rule: "NPWP validation must accept 15 and 16 digits, with optional dots and hyphens"
    covers: [core, validation, tax]
    
  sync.graceful-empty:
    kind: resilience-contract
    rule: "Operational sync downloads on 200 OK must fallback to empty list when data is null"
    covers: [simplidot, sync, download]
```

When an agent flags a violation, it cross-references the registered Oracle ID (e.g. `[VIOLATION: stock.reversibility]`), providing objective, repeatable justification for the required code fix.
