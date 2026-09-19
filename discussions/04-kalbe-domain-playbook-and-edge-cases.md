# 04 — Domain-Specific Test Matrix & Kalbe Playbook

**Document Status:** Approved Discussion Record  
**Target Domain:** Indonesian Enterprise & FMCG Field Sales Applications  
**Date:** September 2026  

---

## 1. Overview

While the QA Swarm architecture is universal, each enterprise ecosystem maintains specific regional, business, and architectural conventions. 

This playbook codifies the exact rules, boundary constraints, and lessons learned from the **Kalbe Nutritionals Super App** deployment (`module/simplidot`, `module/falcon`, `module/core`). It serves as an injection knowledge-base for the agent personas when auditing Kalbe codebases.

---

## 2. Indonesian Regional & Identity Form Controls

| Field Type | Domain Rule | Required Formatter / Validator | Historical Failure Mode |
|---|---|---|---|
| **RT / RW** | Must be strictly numeric, 1 to 3 digits (e.g. `001`, `05`, `12`). Never allows letters or punctuation. | `AppTextFieldType.number`, `FilteringTextInputFormatter.digitsOnly`, `LengthLimitingTextInputFormatter(3)` | User could type string characters (`"abc"`, `"RT 05"`) or exceed 3 digits, failing backend sync validation. |
| **NPWP (Tax ID)** | Supports both legacy 15-digit personal and new 16-digit corporate/NIK Coretax formats. Must tolerate dots (`.`) and hyphens (`-`). Length limit 25 chars formatted. | `FilteringTextInputFormatter.allow(RegExp(r'[0-9.\-\s]'))`, `LengthLimitingTextInputFormatter(25)`, `NpwpValidator` | Over-strict validator rejected formatted NPWPs (`01.234.567.8-901.000`) or valid 16-digit corporate IDs. |
| **Phone Number** | Indonesian mobile standard: starts with `08` or `+628`. Strips leading zero if `+62` prefix is selected. Length 9–14 digits. | `PhoneInputFormatter`, `PhoneValidator.validateMobile` | Double prefix (`+620812...`) or trailing letters accepted in form. |
| **NIK (KTP)** | Exactly 16 numeric digits, encoded with provincial, regional, and birthdate tokens. | `FilteringTextInputFormatter.digitsOnly`, `LengthLimitingTextInputFormatter(16)` | Incomplete 15-digit or 17-digit entries accepted. |
| **Geographic Cascade** | Changing Province must clear City, District, and Sub-District. Changing City must clear District and Sub-District. | Reactive reset callbacks on selection change | Sales rep changed Province but left previous City selected, resulting in an impossible geographic hierarchy. |

---

## 3. Offline-First SQLite / Drift Invariants

```text
┌────────────────────────────────────────────────────────────────────────┐
│                   OFFLINE STATE CONSERVATION RULES                     │
├────────────────────────────────────────────────────────────────────────┤
│ RULE 1: INVENTORY CONSERVATION                                         │
│ Local On-Hand Stock + Outbox Pending Stock == Initial Synced Stock.   │
├────────────────────────────────────────────────────────────────────────┤
│ RULE 2: REVERSIBLE OUTBOX ACTIONS                                      │
│ If an unpushed outbox row is deleted locally:                          │
│   1. Restore deducted inventory to motoris stock table.                │
│   2. Insert an audit entry in StockMutationHistory table.              │
│   3. Delete any child detail records (invoice items, photo refs).      │
├────────────────────────────────────────────────────────────────────────┤
│ RULE 3: AUDIT IDENTITY PERSISTENCE                                     │
│ Every outbox transaction must persist `insertedBy` (Employee Name)     │
│ and `employeeNik` captured directly from the authenticated session.    │
└────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Defensive Resilience Matrix (Backend & Storage)

When working with enterprise legacy backends, client code must adhere to **Defensive Resilience (Graceful Degradation)**:

### 1. The Operational vs. Reference Download Rule
- **Operational Data (Invoices, Customers, Stockist, Motorist Stock):**
  - If backend returns HTTP `200 OK` with `data: null` $\implies$ fallback to `const []`. Never throw `FormatException`. Newly assigned motoris or empty territories must not break the download lifecycle.
- **Reference / Master Data (Provinces, Cities, Product Catalog):**
  - If backend returns `null` $\implies$ throw explicit `FormatException` or trigger an alert. Master tables must not be silently emptied.

### 2. Media Lifecycle Resilience
- Local media files stored in application cache can be reclaimed by Android at any time.
- During sync outbox execution:
  - If `file.exists() == true` $\implies$ attach `MultipartFile`.
  - If `file.exists() == false` $\implies$ **log warning, omit multipart attachment, and proceed with payload sync**.
  - *Never throw `StateError`* that permanently blocks subsequent outbox transactions in the FIFO queue.

### 3. Cross-Boundary Identity
- Do **not** assert equality between SuperApp IAM GUIDs (`txtGuid`) and Domain ERP/Maven GUIDs (`txtMotorisId`).
- Use the employee's **NIK** (`txtPegawaiNik` / `employeeNik`) as the authoritative correlation key across system boundaries.
