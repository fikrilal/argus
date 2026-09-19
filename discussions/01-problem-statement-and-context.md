# 01 — Problem Statement, Engineering Context & Objectives

**Document Status:** Approved Discussion Record  
**Target Domain:** Universal Mobile/Web Applications (Piloted on Kalbe Nutritionals Super App)  
**Date:** September 2026  

---

## 1. Context & Background

An engineer joined Kalbe Nutritionals approximately two months ago, taking full engineering ownership of the **`simplidot`** module within the multi-module Super App monorepo (`falcon_kalbe_app` containing `module/core`, `module/falcon`, `module/fprs`, and `module/simplidot`).

The Simplidot module delivers complex offline-first enterprise distribution capabilities:
- Local offline transaction recording via **Drift (SQLite)**.
- Offline-to-online synchronisation outboxes (Customer Creation, Invoices, Motoris Stock, Stockist Visits / Kulakan).
- Product catalog and image caching.
- GPS/geocoding and OpenStreetMap integration.
- Dynamic business logic tailored to Indonesian General Trade (GT) distribution operations.

Upon completing the core feature scope of the module, the code was handed over to the internal QA team for functional and integration testing.

---

## 2. The Core Problem: The "Builder vs. Breaker" Blind Spot

During QA testing, QA engineers logged a collection of **small-to-medium edge-case issues and behavioral defects**. 

While all major business workflows functioned correctly on the expected happy paths, these subtle gaps resulted in bug reports that negatively impacted the perceived quality of the delivery and posed a direct threat to the engineer's performance KPIs.

### Examples of Issues Discovered in QA:

1. **Form Input Sanitization & Field Boundaries:**
   - In the **Create Outlet / Add Customer Page**, the `RT` and `RW` text fields allowed arbitrary string characters (letters, punctuation) and had no character length cap, despite RT/RW in Indonesia strictly being a 1-to-3 digit administrative number.
   - The **NPWP Validator** initially enforced strict 15 or 16 raw digit lengths without tolerating punctuation or the new Coretax corporate format (such as `01.234.567.8-901.000` or corporate 16-digit extensions), rejecting legitimate merchant tax numbers.

2. **State Invariant & Reversal Gaps:**
   - When a sales invoice was created offline, motoris inventory was deducted locally.
   - However, when a user or supervisor deleted an **unpushed, pending invoice** directly from the local device outbox, the system deleted the outbox row without rolling back the deducted motoris stock back into the local inventory ledger.

3. **Offline-First Resilience & Payload Tolerances:**
   - When operational endpoints (e.g. download invoices, stockist, motorist stock) returned HTTP `200 OK` with `data: null` (common for newly onboarded sales reps with no historical data), the client threw a fatal `FormatException`, marking the entire sync as failed.
   - When an unpushed transaction referenced a locally staged photo/receipt image that was subsequently deleted by Android OS storage optimization, the sync executor threw a fatal `StateError('File tidak ditemukan')`, permanently stalling the FIFO sync queue.

4. **Cross-System Identity Mismatches:**
   - The sync validation compared the IAM SuperApp Employee GUID (`e6d7e4c9...`) with the Simplidot Maven Database Motoris GUID (`2993bba6...`). Because they originate from different database schemas, the client threw `StateError: Data stok motoris tidak sesuai dengan sesi pengguna`, even though the actual employee NIK (`K00013443`) was completely identical and valid.

---

## 3. Root Cause Analysis: Why Developer Unit Tests Miss These Bugs

These bugs were **not** caused by negligence or lack of skill. They stem from a fundamental psychological and structural reality in software engineering:

```
┌───────────────────────────────────────┐       ┌───────────────────────────────────────┐
│           DEVELOPER MINDSET           │       │              QA MINDSET               │
│            ("BUILDER MODE")           │       │           ("BREAKER MODE")            │
├───────────────────────────────────────┤       ├───────────────────────────────────────┤
│ • Focus: Does the feature work?       │  VS   │ • Focus: How can I break this?        │
│ • Tests verify intended functionality │       │ • Tests attack unintended boundaries  │
│ • Assumes cooperative user inputs     │       │ • Assumes hostile / chaotic inputs    │
│ • Assumes reliable system resources   │       │ • Simulates missing files & dropped OS│
│ • Verifies forward progress (A -> B)  │       │ • Verifies reversal (A -> B -> Undo)  │
└───────────────────────────────────────┘       └───────────────────────────────────────┘
```

### The "Golden Path" Testing Bias
In standard practice, developers write **Golden Path Unit Tests**:
```dart
// Standard Developer Test:
test('saves invoice and deducts stock', () async {
  await controller.saveInvoice(validCustomer, [itemA]);
  expect(stockDao.getBalance('itemA'), 7); // Passes!
});
```
The developer does **not** routinely write:
- *What happens if the user deletes the draft 5 seconds later?*
- *What if the user pastes 1,000 characters into the RT field?*
- *What if the camera file gets removed by the OS before the network reconnects?*
- *What if the server returns 200 OK with `null`?*

---

## 4. Engineering Impact & Professional Stakes

In an enterprise environment like Kalbe Nutritionals:
- **KPI Degradation:** Performance evaluations frequently correlate with the defect density reported by QA during formal test cycles.
- **Cycle Time Bloat:** Ping-pong cycles between Developer $\rightarrow$ QA $\rightarrow$ Developer $\rightarrow$ QA waste days for fixes that could have been resolved in minutes prior to handoff.
- **Erosion of Trust:** Trivial bugs (such as typing letters in an RT field) make stakeholders question whether deeper, critical architecture is sound.

---

## 5. Objectives: What Are We Trying to Solve?

We need a system that acts as an **autonomous, adversarial pre-flight QA layer** that runs before code ever reaches a human QA engineer.

### Key Requirements:
1. **Universal Applicability:** Must not be hardcoded solely to Simplidot. The system must operate across Falcon, FPRS, core infrastructure, or any future mobile/web application.
2. **Adversarial & Multi-Perspective:** Must simulate a dozen QA personas simultaneously (Form Saboteur, Invariant Auditor, Chaos User, Network Pessimist).
3. **Non-Golden Testing Paradigms:** Must move beyond happy-path tests to property-based verification, state invariant conservation laws, and chaos lifecycle fuzzing.
4. **Actionable Output:** Must not merely output vague advice; it must produce:
   - Prioritized Defect Lists (P0 Blocker, P1 Major, P2 Edge Case).
   - Concrete code fix recommendations.
   - **Auto-generated, runnable negative Dart tests** that can be committed to the repository test suite.
