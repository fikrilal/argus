# Harness Engineering Design

## Status

Approved on October 1, 2026.

This document defines the engineering harness Argus builds around the codebase so humans and coding agents can make changes safely, quickly, and repeatedly.

It builds on the approved architecture, project structure, design principles, and implementation plan.

Inspired by OpenAI's harness engineering guidance and local best practices from `burnly` and `mobile-core-kit`.

---

## 1. Decision Summary

- The harness is a first-class implementation foundation, not an afterthought.
- Repository-local knowledge is the system of record.
- `AGENTS.md` stays short, authoritative, and points to deeper documentation.
- Non-trivial work uses structured, versioned execution plans under `docs/exec-plans/active/`.
- One canonical `./scripts/verify.sh` mirrors the expected local quality gate.
- Architecture boundaries are enforced mechanically via `tests/architecture_test.rs`.
- Rust uses strict lints: `unsafe_code = "forbid"`, `unwrap_used = "deny"`, `expect_used = "deny"`, `panic = "deny"`.
- Generic names (`helper`, `manager`, `utils`) are prohibited at the compiler/test level.
- Never commit or push without explicit user instruction.

---

## 2. Why Harness Matters for Argus

Argus has several critical boundaries:
- Rust CLI to OS Git subprocesses.
- Rust CLI to asynchronous Pi.dev subprocesses.
- Persona YAML frontmatter to Rust data models.
- Concurrency semaphores and task cancellation.
- User terminal output and file report generation.

Documentation alone will not prevent regressions once implementation accelerates. The harness makes the intended path easy and violations immediately obvious.

---

## 3. Harness Principles

- **Make the repository legible to future agents:** Explicit boundaries and stable naming.
- **Prefer deterministic local checks over reviewer memory:** Machine-checked invariants.
- **Keep checks cheap enough to run frequently:** Local verification passes in $< 1\text{s}$.
- **Promote repeated mistakes into mechanical checks:** If a bug pattern repeats 2+ times, add a lint, test, or oracle.
- **Record execution evidence inside the repository:** Verification evidence is documented in execution plans.
