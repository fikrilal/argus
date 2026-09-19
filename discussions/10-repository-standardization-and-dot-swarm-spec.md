# 10 — Repository Standardization: The `.swarm/` Specification & Dynamic Personas

**Document Status:** Approved Discussion Record  
**Target Domain:** Repository Standardization, Configuration Schema & Persona Dynamic Discovery  
**Date:** September 2026  

---

## 1. Architectural Concept: Repository-Local Swarm Ownership

Every software codebase operates under different business constraints, architectural patterns, and regulatory environments:
- An offline FMCG distribution app (e.g. Kalbe Simplidot) requires inventory rollbacks, Indonesian regional tax formatting, and GPS jitter tolerance.
- A fintech app requires PCI-DSS compliance, currency arithmetic precision, and idempotency key safety.
- A healthcare app requires HIPAA consent audits and zero-leakage local caching.

To support this variety cleanly without polluting the CLI with hardcoded rules, the **QA Swarm adopts a standardized repository-local directory: `.swarm/`**.

---

## 2. Directory Layout Specification

```text
<repository-root>/
├── .swarm/
│   ├── config.yaml               # Active squads, model overrides, SFD location
│   ├── oracles.yaml              # Machine-checked business invariants
│   ├── personas/                 # Specialized persona Markdown prompts
│   │   ├── sfd-clause-detective.md
│   │   ├── id-regulatory-sentinel.md
│   │   ├── stock-ledger-auditor.md
│   │   ├── form-boundary-saboteur.md
│   │   ├── cascade-dropdown-glitcher.md
│   │   ├── concurrency-double-tapper.md
│   │   ├── orphan-cascade-hunter.md
│   │   ├── payload-pessimist.md
│   │   ├── sync-deadlock-guard.md
│   │   ├── hardware-sensor-adversary.md
│   │   ├── rbac-identity-gatekeeper.md
│   │   └── lead-qa-synthesizer.md
│   └── context/                  # Project specifications & domain reference docs
│       └── sfd/
│           └── stockist-kulakan.md
```

---

## 3. Two-Tier Persona Inheritance

To ensure developers do not have to write boilerplate personas from scratch for every new project, the system implements **Two-Tier Inheritance**:

```text
┌────────────────────────────────────────────────────────────────────────┐
│                   TIER 1: BUILT-IN CORE PERSONAS                       │
│  Shipped with the CLI package (Universal Best-Practices):              │
│  • form-boundary-saboteur      • concurrency-double-tapper             │
│  • payload-pessimist           • orphan-cascade-hunter                 │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │
                                    ▼ Merged & Overridden by
┌────────────────────────────────────────────────────────────────────────┐
│                   TIER 2: PROJECT-LOCAL `.swarm/`                      │
│  Defined inside the target repository:                                 │
│  • Registers project-specific personas (e.g. id-regulatory-sentinel)   │
│  • Overrides base persona instructions with repository-specific rules  │
│  • Defines custom squad groupings and active SFD context               │
└────────────────────────────────────────────────────────────────────────┘
```

### Resolution Rules:
1. **Fallback:** If a persona is invoked but does not exist in `.swarm/personas/`, the CLI falls back to the built-in system persona.
2. **Override:** If `.swarm/personas/<name>.md` exists, it completely overrides the built-in persona with project-specific instructions.
3. **Extension:** Any new `.md` file placed in `.swarm/personas/` is automatically discovered and registered as a valid agent persona.

---

## 4. Configuration Schema: `.swarm/config.yaml`

```yaml
version: 1
project: "kalbe-superapps"

# Specification Document configuration
sfd:
  path: ".swarm/context/sfd/"
  active: "stockist-kulakan.md"

# Model Tier Mapping for cost & reasoning optimization
models:
  fast: "google/gemini-2.5-flash"         # for simple text field & regex scans
  standard: "anthropic/claude-3-7-sonnet" # for general adversarial review
  deep: "anthropic/claude-3-7-sonnet:high" # for complex SFD & state reversibility

# Pre-configured Squads for targeted audits
squads:
  forms:
    - id-regulatory-sentinel
    - form-boundary-saboteur
    - cascade-dropdown-glitcher
  state:
    - stock-ledger-auditor
    - orphan-cascade-hunter
  sync:
    - payload-pessimist
    - sync-deadlock-guard
    - hardware-sensor-adversary
  pre-qa:
    - all
```

---

## 5. Persona Prompt Specification: Frontmatter & Structure

Each persona is authored as a Markdown document with YAML frontmatter:

```markdown
---
name: stock-ledger-auditor
title: The Stock & Local Ledger Invariant Auditor
squad: state
model_tier: deep
tools: read, grep, find, ls, bash
---

You are an adversarial database and state machine auditor. Your sole mission is to ensure that every local data mutation obeys the Reversibility Law: Delta(State) = 0.

## What you attack:
1. Examine all Drift/SQLite DAOs and Outbox tables modified in the diff.
2. If an entity is created and deducts inventory (e.g., `SimplidotInvoiceOutboxDao`), verify that deleting that unpushed entity has a symmetric rollback method that restores the stock.
3. Check for orphan records: if a parent header row is deleted, are child items and staged file paths cleaned up?

## Output Format:
- **Status:** [VIOLATION / PASS]
- **Target:** Affected file and line number
- **Rule Breached:** Reference to oracle or reversibility law
- **Failure Scenario:** Step-by-step reproduction
- **Recommended Fix:** Drift SQL / Dart transaction code
```

---

## 6. Developer CLI Ergonomics

```bash
# Scaffold a clean .swarm/ in any new repository
qaswarm init

# Run only form-related audit (fast, uses Tier 1 + 2 personas)
qaswarm audit --squad forms

# Run pre-QA full audit with active SFD cross-referencing
qaswarm audit --all --sfd .swarm/context/sfd/stockist-kulakan.md

# Resume an agent session interactively
pi -r qa-swarm/<branch>/stock-ledger-auditor
```
