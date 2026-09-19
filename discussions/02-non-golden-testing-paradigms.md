# 02 — Beyond Happy Paths: Non-Golden Testing Paradigms

**Document Status:** Approved Discussion Record  
**Target Domain:** Universal Software Testing Theory & Application  
**Date:** September 2026  

---

## 1. The Fallacy of Golden Path Testing

Traditional Unit and Integration Testing relies almost universally on **Golden Path Specifications**:
> **Definition:** Given a known, well-formed input $X_{valid}$, execute function $F$, and assert that the output equals expected result $Y_{ideal}$.

While essential for confirming that features meet basic business requirements, golden path testing exhibits near-zero sensitivity to:
- Input domain boundaries and illegal permutations.
- Reversal or rollback lifecycle invariants.
- Dynamic degradation when external dependencies behave unexpectedly.

To catch defects before human QA, an automated pre-flight system must introduce **Non-Golden Testing Paradigms**.

---

## 2. The Three Non-Golden Testing Pillars

```
┌────────────────────────────────────────────────────────────────────────┐
│                      NON-GOLDEN TESTING METHODOLOGY                    │
├─────────────────────┬──────────────────────┬───────────────────────────┤
│     PILLAR 1:       │      PILLAR 2:       │         PILLAR 3:         │
│   PROPERTY-BASED    │    METAMORPHIC &     │       STATE MACHINE       │
│      TESTING        │   INVARIANT LAWS     │          CHAOS            │
├─────────────────────┼──────────────────────┼───────────────────────────┤
│ Feed 1,000+ pseudo- │ Assert algebraic     │ Execute non-linear,       │
│ random, adversarial │ conservation laws    │ shuffled action sequences │
│ inputs to discover  │ across state         │ to expose race conditions │
│ unhandled crashes.  │ transitions.         │ and re-entrancy bugs.     │
└─────────────────────┴──────────────────────┴───────────────────────────┘
```

---

### Pillar 1: Property-Based Testing (Generative / QuickCheck Testing)

Instead of hand-crafting 3 static test cases (`'01.234.567.8-901.000'`, `'3201012345678901'`, `''`), **Property-Based Testing** defines invariant properties that must hold true for *any* conceivable value in the type domain.

#### Characteristics:
1. **Generative Input:** An automated generator synthesizes hundreds of dirty strings (Unicode emojis, null bytes `\0`, RTL text, whitespace sequences, 10,000-character payloads, SQL injection payloads).
2. **Shrinking:** When a failure occurs (e.g. at character count 4,129), the engine automatically shrinks the test case down to the minimal reproducible trigger (e.g. character `#` at position 3).
3. **The Crash-Proof Property:**
   $$\forall s \in \text{String}: \text{evaluate}(s) \in \{\text{Valid}, \text{Invalid}\} \quad \land \quad \text{throw}(\text{UnhandledException}) = \text{False}$$

#### Application in Dart / Flutter:
Using libraries such as `glados`:
```dart
Glados<String>().test('NpwpValidator never throws unexpected exceptions', (arbitraryString) {
  expect(
    () => NpwpValidator.validate(arbitraryString),
    returnsNormally,
    reason: 'Validator crashed on input: $arbitraryString',
  );
});
```

---

### Pillar 2: Metamorphic & State Invariant Testing

A program state is governed by business invariants. Even when intermediate state changes, aggregate conservation laws must never be violated.

#### 1. The Reversibility Law ($\Delta \text{State} = 0$)
Any transaction that can be created locally prior to remote sync must be strictly reversible.
$$\text{State}_0 \xrightarrow{+\text{Create}(\text{Entity})} \text{State}_1 \xrightarrow{+\text{Delete}(\text{Entity})} \text{State}_2 \implies \text{State}_2 \equiv \text{State}_0$$

* **Concrete Failure in Simplidot:**
  1. Initial: Stock of Item A = 10 pcs.
  2. Create Invoice for 3 pcs: Stock of Item A = 7 pcs; Outbox has 1 invoice.
  3. Delete Unpushed Invoice: Outbox has 0 invoices; Stock of Item A **remained 7 pcs** (Invariant Violated! $\Delta \text{Stock} = -3$).

#### 2. The Conservation Invariant
In offline distribution, inventory must be conserved across local buckets:
$$\text{Physical On-Hand Stock} + \sum(\text{Outbox Pending Deduction}) = \text{Total Assigned Stock}$$

Metamorphic tests verify these relationships by asserting state equality across complementary operations rather than checking hardcoded absolute values.

---

### Pillar 3: State Machine Chaos & Lifecycle Fuzzing

Real-world users and mobile operating systems do not execute linear workflows. Chaos testing introduces non-determinism into the application lifecycle:

```
[Normal Path]   : Screen A ──> Fill ──> Submit ──> Await Response ──> Success
[Adversarial]   : Screen A ──> Fill ──> Double-Tap Submit ──> Navigate Back ──> Kill Process
```

#### Key Chaos Vectors to Verify:
1. **Re-Entrancy / Double-Tap:**
   - What happens if the user taps "Simpan" or "Bayar" 3 times within 150 milliseconds?
   - *Requirement:* Button state disables immediately or controller debounces; only 1 outbox record is persisted.
2. **Mid-Flight Disruption:**
   - Navigating backwards (`Get.back()`) while an asynchronous database transaction or geocoding call is pending.
   - *Requirement:* Controller checks `isClosed` / `mounted`; does not attempt reactive updates on disposed bindings.
3. **Process Termination Recovery:**
   - If Android terminates the app process after saving an outbox row but before copying an image to permanent storage, what state does the app boot into?
   - *Requirement:* Database recovers cleanly; missing media triggers graceful sync fallback instead of a fatal crash.

---

## 3. Comparison Summary

| Metric | Golden Path Unit Testing | Non-Golden Testing |
|---|---|---|
| **Primary Goal** | Prove that happy path features work | Prove where and how the system breaks |
| **Input Generation** | Hand-crafted by the developer | Generative, fuzzing, property-derived |
| **Lifecycle Scope** | Forward progression only ($A \rightarrow B$) | Cyclic and reversible ($A \rightarrow B \rightarrow \text{Cancel}$) |
| **Failure Assumption** | Assumes cooperative backend & OS | Assumes hostile inputs, missing files, null data |
| **Value to QA** | Validates developer specification | Eliminates 80%+ of QA-discovered edge cases |
