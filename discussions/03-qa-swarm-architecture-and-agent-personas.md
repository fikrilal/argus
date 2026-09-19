# 03 — Universal QA Swarm Architecture & Agent Personas

**Document Status:** Approved Discussion Record  
**Target Domain:** Multi-Agent Architecture for Autonomous Pre-Flight QA  
**Date:** September 2026  

---

## 1. Architectural Vision: The Universal Red Team Swarm

The goal of the **QA Swarm** is to replace passive code review with an active, multi-perspective **adversarial audit**. 

Instead of a single LLM giving general feedback, the Swarm deploys **specialized agent personas**, each possessing an obsessive, narrow focus on a specific failure mode. Because these failure modes reflect universal software patterns (input sanitization, state conservation, network reliability, and lifecycle concurrency), the architecture is **100% project-agnostic**—applicable equally to Simplidot, Falcon, FPRS, or any future software product.

---

## 2. Swarm Topology & Workflow

```text
                           Developer Workspace
                        (Git Diff / Staged Changes)
                                    │
                                    ▼
       ┌────────────────────────────────────────────────────────┐
       │             ORCHESTRATOR & CONTEXT INJECTOR            │
       │  • Extracts modified files, schemas, and UI components │
       │  • Dispatches diff to specialized agent workers       │
       └────────────────────────────┬───────────────────────────┘
                                    │ Parallel Dispatch
         ┌──────────────┬───────────┴───────────┬──────────────┐
         ▼              ▼                       ▼              ▼
   ┌───────────┐  ┌───────────┐           ┌───────────┐  ┌───────────┐
   │  AGENT 1  │  │  AGENT 2  │           │  AGENT 3  │  │  AGENT 4  │
   │   Input   │  │   State   │           │  Network  │  │  Lifecycle│
   │ Boundary  │  │ Invariant │           │  & Desync │  │  & Chaos  │
   │ Saboteur  │  │  Hunter   │           │ Pessimist │  │   Abuser  │
   └─────┬─────┘  └─────┬─────┘           └─────┬─────┘  └─────┬─────┘
         │              │                       │              │
         └──────────────┼───────────────────────┴──────────────┘
                        │ Individual Audit Findings
                        ▼
       ┌────────────────────────────────────────────────────────┐
       │                        AGENT 5                         │
       │             LEAD QA SYNTHESIZER & ARCHITECT            │
       │  • Deduplicates & cross-references findings            │
       │  • Assigns Severity (P0 Blocker, P1 Major, P2 Edge)   │
       │  • Produces Pre-Flight QA Actionable Defect Report     │
       │  • Generates Runnable "Non-Golden" Dart Tests          │
       └────────────────────────────┬───────────────────────────┘
                                    │
                                    ▼
                          Developer Fix & Verify
                        (`flutter test` Passes)
                                    │
                                    ▼
                         Zero-Defect QA Handoff
```

---

## 3. Deep Dive: The 5 Specialized Agent Personas

### Agent 1: The Input Boundary Saboteur ("The Form Pest")
* **Adversarial Mindset:** *"I assume all human input is malicious, mistaken, or formatted weirdly. If there is a way to paste bad data, I will find it."*
* **Core Audit Checks:**
  1. **Missing Formatters:** Examines every `TextField` / `AppTextField`. Flags any text input lacking an explicit `inputFormatters` or `LengthLimitingTextInputFormatter`.
  2. **Domain Boundary Violations:** Identifies Indonesian or domain-specific fields (e.g. RT/RW, NIK, NPWP, Phone, Postal Code) that do not enforce strict character-set filtering.
  3. **Zero & Negative Value Permutations:** In numeric inputs (Quantity, Price, Discount, Stock), checks what occurs if the user enters `0`, `-1`, or values exceeding total capacity.
  4. **Cascade Invalidation:** Checks multi-level dropdowns (Province $\rightarrow$ City $\rightarrow$ District $\rightarrow$ Subdistrict). Verifies that changing a parent picker immediately resets dependent children to prevent invalid composite geographic references.

---

### Agent 2: The State Invariant Hunter ("The Ledger Auditor")
* **Adversarial Mindset:** *"Every action has an equal and opposite reaction. If an operation changes state, there must be a mathematically sound reversal mechanism."*
* **Core Audit Checks:**
  1. **The Reversal / Rollback Audit:** Scans all `delete()`, `cancel()`, `clear()`, or `back()` operations across DAOs and controllers. Confirms that local side effects (such as deducted inventory or altered status flags) are restored via compensatory mutations.
  2. **Single Source of Truth:** Flags cached duplicate state variables across controllers that can drift away from underlying database tables.
  3. **Orphaned Row Detection:** Verifies cascade deletes across local relational tables (e.g. if an invoice outbox row is deleted, are related detail line items and staged media references cleaned up or left as ghost records?).

---

### Agent 3: The Network & Desync Pessimist ("The Murphy's Law Agent")
* **Adversarial Mindset:** *"The backend is unpredictable. Endpoints will return nulls, network connections will drop mid-request, and local files will disappear."*
* **Core Audit Checks:**
  1. **Null Payload Resilience:** Identifies any repository method that assumes `response.data` is non-null on HTTP `200 OK`. Flags unsafe force-unwraps (`!`) and non-graceful `FormatException` throws on operational collections.
  2. **Missing Local Media:** Checks sync outbox workers that upload attachments (`MultipartFile`). Ensures that if a local cached file was deleted by the user or OS storage cleaner, the sync continues without stalling the queue.
  3. **Cross-System ID Assumptions:** Flags code that asserts foreign database GUIDs are equal across disparate backend boundaries (e.g. IAM GUID vs ERP / Maven GUID).
  4. **Idempotent Retry Safety:** Ensures that re-executing a failed sync upload does not produce duplicate server-side records.

---

### Agent 4: The Chaos & Lifecycle Abuser ("The Impatient User")
* **Adversarial Mindset:** *"I smash buttons repeatedly, I leave screens while data is loading, and I swap apps without waiting."*
* **Core Audit Checks:**
  1. **Re-Entrancy & Debounce:** Checks whether button callbacks disable themselves immediately upon click. Disallows multiple in-flight asynchronous operations triggered by fast double-tapping.
  2. **Disposed Binding Safety:** Audits asynchronous `await` chains in GetX controllers to ensure state updates check `isClosed` or component mounting before executing reactive mutations.
  3. **Offline / Flight-Mode Toggling:** Inspects background sync triggers (`Workmanager`) to ensure graceful suspension and resumption when connectivity fluctuates.

---

### Agent 5: The Lead QA Synthesizer & Test Architect
* **Mission:** Synthesizes the raw attack findings from Agents 1–4 into an authoritative, actionable handoff package.
* **Responsibilities:**
  1. **Deduplication:** Merges overlapping findings into single architectural issues.
  2. **Severity Triaging:**
     - **P0 (Blocker):** Data loss, unrecoverable crashes, sync queue deadlock, invariant breakage (e.g. lost stock).
     - **P1 (Major):** Bypassed form validations that send invalid data to the backend, cascading dropdown bugs.
     - **P2 (Minor / Polish):** Missing visual feedback, lack of soft length limits, minor UI formatting quirks.
  3. **Automated Test Generation:** Generates standalone, runnable Dart test files targeting the discovered vulnerabilities directly.
