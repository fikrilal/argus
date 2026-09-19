# 07 — Knowledge Asymmetry, SFD Ground Truth & Resumable Pi Sessions

**Document Status:** Approved Discussion Record  
**Target Domain:** Architecture Redesign around Knowledge Gaps, Spec Cross-Referencing & Native Pi Sessions  
**Date:** September 2026  

---

## 1. The Root Cause Breakthrough: Knowledge Asymmetry

A fundamental insight was established in this discussion: the defects escaping to QA were **not caused by poor code quality or syntax bugs**. Instead, they were caused by **Knowledge Asymmetry**—the gap between what a new engineer knows (~2 months in the company) versus the domain expertise accumulated by System Analysts (SA) and senior QA over years.

### The Three Failure Modes of Knowledge Asymmetry:
1. **"I Forgot" (Omission):** Skimming a 40+ page specification document leads to cognitive fatigue; subtle requirement sub-clauses (e.g. *Section 4.3 note 2: "Unpushed invoice deletion must restore inventory"*) get missed during implementation.
2. **"I Was Not Aware" (State & Lifecycle Symmetry):** Building the creation flow thoroughly while remaining unaware that the business requires an identical, reverse state-machine flow for cancellation, draft deletion, and local inventory rollback.
3. **"I Didn't Know" (Domain & Regulatory Nuances):** Not knowing that Indonesian tax law (DJP Coretax) treats corporate NPWPs differently from personal NPWPs, or assuming standard text fields can be used for administrative regional IDs like RT/RW.

---

## 2. Rejection of Toy Subagents in Favor of Resumable Pi Sessions

The built-in Pi subagent extension (`@earendil-works/pi-coding-agent/examples/extensions/subagent`) was evaluated and **explicitly rejected** for the following architectural reasons:

| Limitation of Subagent Extensions | Advantage of Native Resumable Pi Sessions |
|---|---|
| **Ephemeral & Throwaway:** Subagents execute a prompt and terminate; their context is destroyed. | **Persistent Memory:** Sessions retain conversation history, code context, and ongoing domain notes. |
| **No Interactive Dialogue:** The developer cannot converse with the agent to ask follow-up questions. | **Resumable via `pi -r`:** The developer can run `pi -r qa/sfd-analyst` at any time to pair-program on a fix. |
| **Strict Process / Concurrency Caps:** Hardcoded maximums (e.g. 4–8 subagents). | **Unbounded Scaling:** The developer or runner script can launch dozens of named Pi sessions simultaneously. |

### The Native Resumable Session Pattern:
```bash
# Launching dedicated, persistent sessions in Pi
pi --name "qa/sfd-analyst" --system-prompt @agents/sfd_analyst.md
pi --name "qa/id-compliance" --system-prompt @agents/id_compliance.md
pi --name "qa/state-architect" --system-prompt @agents/state_architect.md
pi --name "qa/chaos-tester" --system-prompt @agents/chaos_tester.md

# Resuming at any time to ask questions or refine logic
pi -r "qa/state-architect"
```

---

## 3. The SFD (System Functional Design) as the Ground Truth Engine

At Kalbe Nutritionals, every major feature and module is governed by an **SFD (System Functional Design)** document prepared by the System Analyst (SA).

### The Ingestion Pipeline:
- **Source Formats:** SFDs are delivered in PDF (`.pdf`) or Microsoft Word (`.docx`).
- **Conversion Strategy:** The engineer converts the SFD into clean Markdown (`.md`).
- **Why Markdown Conversion is Superior:**
  - Token-efficient and zero formatting noise.
  - Native semantic parsing for LLM agents (headings, tables, requirement lists).
  - Can be placed directly in the repository or session context (`@docs/sfd/stockist-kulakan.md`).

### Human Reading vs. LLM Spec Cross-Referencing:
- **A human engineer** reading a 45-page Word document suffers from cognitive blind spots and focuses on the UI mockups and happy paths.
- **An LLM agent** cross-referencing an SFD against a `git diff` performs exhaustive semantic matching with zero fatigue, comparing every clause of the spec against every line of modified code:
  > *"Warning: SFD Section 3.2.1 states: 'Pada pembatalan/penghapusan faktur lokal yang belum tersinkronisasi, stok motoris harus dikembalikan sejumlah kuantiti faktur.' Your diff in `simplidot_invoice_outbox_dao.dart` calls `delete()`, but does not invoke `restoreStockMutation()`."*

---

## 4. The 4 Knowledge-Driven Swarm Pillars

To bridge the "Forgot", "Not Aware", and "Don't Know" gaps, the swarm is organized into four core intelligence pillars:

```
┌────────────────────────────────────────────────────────────────────────┐
│                        THE 4 KNOWLEDGE PILLARS                         │
├──────────────────┬──────────────────┬──────────────────┬───────────────┤
│   PILLAR A:      │    PILLAR B:     │    PILLAR C:     │   PILLAR D:   │
│   THE SFD &      │   INDONESIAN     │  ARCHITECTURAL   │    FIELD &    │
│  BUSINESS SPEC   │   REGULATORY     │   SYMMETRY &     │    NETWORK    │
│    AUDITOR       │   COMPLIANCE     │     LEDGER       │     CHAOS     │
├──────────────────┼──────────────────┼──────────────────┼───────────────┤
│ Solves what you  │ Solves what you  │ Solves what you  │ Solves what   │
│ "FORGOT" from    │ "DIDN'T KNOW"    │ "ASSUMED" about  │ you assumed   │
│ the 40-page SA   │ about Indonesian │ state reversal   │ about stable  │
│ design document. │ legal/regional   │ and inventory    │ connections & │
│                  │ data rules.      │ balance laws.    │ clean servers.│
└──────────────────┴──────────────────┴──────────────────┴───────────────┘
```

### Pillar A: The SFD & Business Requirement Auditor
* **Core Function:** Cross-references the converted Markdown SFD against `git diff`.
* **Solves:** *"I forgot to implement requirement 3.4 on page 27."*
* **Outputs:** A checklist of implemented requirements versus missing sub-clauses, unhandled business exceptions, and omitted status flags.

### Pillar B: The Indonesian Regional & Regulatory Expert
* **Core Function:** Enforces external legal, tax, telco, and regional standards that are often unwritten in internal specs.
* **Solves:** *"I didn't know corporate NPWP had a different format under Coretax, or that RT/RW must be capped at 3 digits."*
* **Outputs:** Warnings on form text fields, validator gaps, missing input formatters, and formatting regex vulnerabilities.

### Pillar C: The Architectural Symmetry & Ledger Auditor
* **Core Function:** Audits state mutations for mathematical and operational reversibility.
* **Solves:** *"I didn't think about what happens when an unpushed local record is deleted or cancelled."*
* **Outputs:** Rollback verification across SQLite/Drift DAOs, ensuring every deduction has a compensatory return mutation and zero orphaned child rows.

### Pillar D: The Field & Network Chaos QA
* **Core Function:** Simulates hostile real-world mobile environments (traditional markets, intermittent 3G, aggressive Android OS memory reclaiming).
* **Solves:** *"I tested with clean mock data on high-speed emulator WiFi."*
* **Outputs:** Audits for HTTP 200 `data: null` crashes, missing media file recovery, and cross-system identifier mismatches (e.g. IAM GUID vs Maven DB GUID).

---

## 5. End-to-End Workflow in Daily Engineering

```text
Step 1: Pre-Implementation Ingestion (Orientation)
        Engineer converts SFD (.docx/.pdf) to Markdown.
        Engineer feeds SFD to `qa/sfd-analyst`:
        > "Extract the Business Rules, State Transitions, and Validation Matrix."
        Agent produces a clear implementation checklist before code is written.

Step 2: Normal Implementation & Unit Testing
        Engineer builds features and writes happy-path tests.

Step 3: Pre-Handoff Swarm Audit
        Swarm runs across the feature branch / git diff:
        - `qa/sfd-analyst` checks for missed requirements.
        - `qa/id-compliance` checks for Indonesian form & tax flaws.
        - `qa/state-architect` checks for missing rollbacks and invariant leaks.
        - `qa/chaos-tester` checks for null handling and offline resilience.

Step 4: Interactive Pair-Refinement via Resumable Sessions
        Engineer opens specific session: `pi -r "qa/state-architect"`
        > "How do I implement atomic stock rollback in Drift for this DAO?"
        Agent provides exact Drift SQL transaction code.

Step 5: Zero-Defect QA Handoff
        Code is submitted to human QA. All edge cases previously logged as bugs
        are already verified, tested, and resolved.
```
