# 06 — QA Swarm Mechanics & Pi Harness Integration

**Document Status:** Approved Discussion Record  
**Target Domain:** Multi-Agent Subagent Architecture in Pi  
**Date:** September 2026  

---

## 1. What Exactly is the "QA Swarm"?

The **QA Swarm** is an automated multi-agent red-teaming system. Instead of asking a single general-purpose AI to "review this code and find bugs", the Swarm delegates the task to a **coordinated team of specialized adversarial subagents**, each running in its own **isolated context window** with a dedicated persona and attack playbook.

### Why a Swarm Outperforms a Single Agent:
1. **Persona Purity & Cognitive Focus:** A single prompt cannot effectively maintain 50 conflicting evaluation rules simultaneously. A dedicated "Input Saboteur" agent has a single mission: finding ways to break input boundaries.
2. **Context Window Isolation:** A single agent reviewing a large diff consumes massive token context, causing it to lose attention on edge cases. In a swarm, each subagent inspects the diff independently with zero cross-contamination.
3. **Parallel Execution:** Agents execute concurrently, reducing audit turnaround time from minutes to seconds.
4. **Adversarial Triangulation:** Different agents view the exact same code from opposing perspectives (e.g. the Form Saboteur attacks the UI field, while the Ledger Auditor attacks the database DAO).

---

## 2. Native Harness Support: How Pi Implements Subagents & Swarms

The current runtime harness (**Pi**) includes a built-in **subagent extension** located at:
`~/.pi/agent/install/releases/0.99.1/node_modules/@earendil-works/pi-coding-agent/examples/extensions/subagent`

### Core Capabilities of the Pi Subagent Engine:
- **Subprocess Isolation:** Each subagent executes in a dedicated `pi` subprocess with isolated memory and tailored tool allowlists (`read`, `grep`, `find`, `ls`, `bash`).
- **Parallel Dispatch:** Supports concurrent execution of up to 8 subagents simultaneously via `{ tasks: [...] }`.
- **Workflow Chaining:** Supports sequential agent pipelines via `{ chain: [...] }`, passing outputs from worker agents into a synthesizer.
- **Declarative Agent Definitions:** Agents are defined as standard Markdown files with YAML frontmatter.

---

## 3. The Declarative Agent Specification in Pi

In Pi, an agent definition resides in `~/.pi/agent/agents/<name>.md` (global) or `.pi/agents/<name>.md` (project-local):

```markdown
---
name: qa-input-saboteur
description: Adversarial form input, boundary, and regex validation tester
tools: read, grep, find, ls, bash
---

You are an adversarial QA specialist focused exclusively on input fields, boundaries, and validation formatting.

Your task is to inspect the git diff and find every input field, regex formatter, numeric limit, and cascade dropdown.
Attack every boundary:
- Can letters be typed into numeric fields?
- Are lengths capped?
- Can negative or zero values be submitted?
- Do cascading pickers reset children?

Be specific with file paths and line numbers.
```

---

## 4. The Complete Execution Lifecycle

```text
Step 1: Developer finishes changes (working branch or staged diff).
                             │
                             ▼
Step 2: Subagent Tool dispatches 4 parallel adversarial tasks:
        ┌────────────────────────────────────────────────────────┐
        │ tasks: [                                               │
        │   { agent: "qa-input-saboteur",   task: "Audit diff" },│
        │   { agent: "qa-invariant-hunter", task: "Audit diff" },│
        │   { agent: "qa-network-pessimist",task: "Audit diff" },│
        │   { agent: "qa-chaos-abuser",     task: "Audit diff" } │
        │ ]                                                      │
        └────────────────────┬───────────────────────────────────┘
                             │
                             ▼ (Parallel Execution in Isolated Subprocesses)
Step 3: Outputs collected and piped to:
        ┌────────────────────────────────────────────────────────┐
        │ { agent: "qa-lead-synthesizer",                        │
        │   task: "Synthesize findings & generate Dart tests" }  │
        └────────────────────┬───────────────────────────────────┘
                             │
                             ▼
Step 4: Developer receives:
        1. Actionable Pre-Flight QA Bug Report.
        2. Auto-generated executable negative tests (*_adversarial_test.dart).
```
