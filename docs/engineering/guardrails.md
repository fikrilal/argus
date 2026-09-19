# Argus Guardrails & Mechanical Constraints

This document defines the mechanical guardrails that keep changes to the **Argus** codebase consistent, reviewable, and production-safe.

---

## 1. Core Principles

Guardrails make the correct path the easiest path. Prefer guardrails that are:
- Deterministic and machine-checked by compilers and linters.
- Fast enough to run locally before every commit.
- Inflexible against unhandled runtime panics.

---

## 2. Hard Language Rules (Rust)

1. **Zero Production Panics (`no_unwrap_policy`):**
   - `.unwrap()` and `.expect()` are **strictly forbidden** in `src/` production paths.
   - Use Rust's `?` operator, `Option::ok_or_else`, or explicitly handle error branches using `anyhow::Context` for rich diagnostic context.
   - `.unwrap()` is allowed only in test functions (`#[test]`).

2. **Zero `unsafe` Code:**
   - The entire codebase must be 100% safe Rust. No `unsafe { ... }` blocks permitted.

3. **Compiler & Clippy Hygiene:**
   - Every build must pass `cargo clippy --all-targets --all-features -- -D warnings`.
   - Never use `#[allow(...)]` to silence warnings without an explicit documented justification approved by the repository owner.

4. **Formatting Discipline:**
   - Code must be formatted using official `rustfmt` (`cargo fmt --check`).

---

## 3. Subprocess & System Guardrails

1. **Subprocess Isolation:**
   - When invoking `pi` or `git`, always use asynchronous `tokio::process::Command`.
   - Never block Tokio worker threads with synchronous `std::process::Command`.

2. **Concurrency Bounds:**
   - Subprocess spawning must be throttled via `tokio::sync::Semaphore`.
   - Unbounded process spawning is forbidden to prevent OS resource exhaustion and API rate-limiting.

3. **Graceful Subprocess Degradation:**
   - If a single `pi` subagent session times out or exits with a non-zero code, the orchestrator must capture the failure as an advisory finding and continue executing the remaining agents. It must never terminate the entire swarm abruptly.

---

## 4. Verification Profiles

### Fast Local Gate:
```bash
cargo check
cargo test
```

### Full Verification Gate:
```bash
./scripts/verify.sh
```

Which executes:
1. `cargo fmt --check`
2. `cargo clippy --all-targets --all-features -- -D warnings`
3. `cargo test`
4. `cargo build --release`
