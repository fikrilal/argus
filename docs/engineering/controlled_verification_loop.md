# Controlled Verification Loop

The delivery cycle for all changes to **Argus** follows a structured, disciplined loop:

```text
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│  1. PLAN     │ ──> │ 2. IMPLEMENT │ ──> │  3. VERIFY   │
└──────────────┘     └──────────────┘     └──────┬───────┘
                                                 │
                                           Pass? │ Fail?
                                      ┌──────────┴──────────┐
                                      ▼                     ▼
                               ┌──────────────┐      ┌──────────────┐
                               │  5. COMMIT   │      │  4. REPAIR   │
                               └──────────────┘      └──────────────┘
```

---

## 1. Plan
- Select an atomic task from [`TODO.md`](../../TODO.md).
- Ensure the task scope is bounded and independent.
- Identify the affected files and unit tests.

## 2. Implement
- Write minimal, focused Rust code satisfying the acceptance criteria.
- Adhere strictly to the [`guardrails.md`](./guardrails.md) rules (zero `unwrap()`, zero `unsafe`).

## 3. Verify
- Run targeted tests for the modified module:
  ```bash
  cargo test <module_name>
  ```
- Run the full verification suite:
  ```bash
  ./scripts/verify.sh
  ```
- Inspect both exit codes and output messages. Never assume success without verifying.

## 4. Repair (If Verification Fails)
- Read compiler diagnostics, Clippy warnings, or failed test assertions carefully.
- Repair the root cause directly; do not bypass with `#[allow(...)]` or test weakenings.
- Re-run verification until all checks pass cleanly.

## 5. Commit
- Stage only the relevant, task-owned files.
- Commit using Conventional Commits with scope:
  ```text
  feat(git): implement diff noise filter for generated files
  test(persona): add frontmatter parsing unit tests
  fix(runner): handle missing pi executable gracefully
  ```
- Leave unrelated or temporary files uncommitted.
