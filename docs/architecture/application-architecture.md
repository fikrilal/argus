# Argus Application Architecture

## Status

Approved on October 1, 2026.

This document defines the architectural foundations of **Argus**, translating our engineering proposal into concrete boundaries, data flows, and concurrency models.

## Architectural Style: Three-Tier Modular Monolith

Argus is designed as a focused, modular CLI binary that orchestrates external AI agents without coupling itself to any single programming language or framework.

```text
┌────────────────────────────────────────────────────────┐
│                   TARGET REPOSITORY                    │
│      Git Working Tree • .argus/ Config • SFD Spec      │
└───────────────────────────┬────────────────────────────┘
                            │ Reads Diff & Context
                            ▼
┌────────────────────────────────────────────────────────┐
│                   ARGUS RUST ENGINE                    │
│  Git Filter ──> Persona Registry ──> Tokio Pool ──> UI │
└───────────────────────────┬────────────────────────────┘
                            │ Spawns Bounded Subprocesses
                            ▼
┌────────────────────────────────────────────────────────┐
│                  PI SUBPROCESS FLEET                   │
│   sfd-clause-detective ... stock-ledger-auditor        │
│          └───> Piped to lead-qa-synthesizer            │
└────────────────────────────────────────────────────────┘
```

## Core Architectural Invariants

1. **Information Hiding:**
   - Raw Git diff parsing and regex filtering are hidden inside `src/git/`. Callers only see clean `GitDiffResult` structs.
   - Persona discovery (built-in vs. local `.argus/` overrides) is hidden inside `src/persona/`. Callers query squads or names.
   - Subprocess execution (`pi -p`) is hidden inside `src/runner/`. Callers pass an `AgentExecutionPlan` and receive an `AgentRunResult`.

2. **Bounded Concurrency & Memory Safety:**
   - All subprocess launches are throttled via an asynchronous `tokio::sync::Semaphore`.
   - The CLI maintains a fixed memory footprint ($< 10\text{MB}$ RAM), never loading arbitrary multi-gigabyte repository trees into memory.

3. **Fault-Tolerant Swarm Execution:**
   - Failure of one agent process (e.g. process timeout, network hiccup) does not terminate the remaining swarm agents.
   - The Lead Synthesizer acknowledges partial runs and synthesizes findings from all available reports.

4. **Symmetric Reversibility:**
   - Every agent session is named deterministically: `argus/<branch>/<persona>`.
   - State is stored in native Pi session files (`~/.pi/agent/sessions/`), ensuring the developer can resume any agent via `pi -r`.

## Non-Goals

- An in-process LLM inference engine (Argus relies on Pi.dev CLI).
- A general code generator or AST refactoring suite (Argus reports defects and stubs, leaving architectural changes to the engineer).
- A persistent daemon or background service.
