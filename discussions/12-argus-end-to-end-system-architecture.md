# 12 — Argus End-to-End System Architecture

**Document Status:** Approved Architectural Record  
**Target Domain:** Complete Rust Orchestrator, Three-Tier Topology & Data Pipeline  
**Binary Name:** `argus`  
**Repository:** `fikrilal/argus`  
**Date:** October 2026  

---

## 1. High-Level System Topology

Argus is designed around a decoupled, three-tier architecture separating the target repository, the native Rust execution engine, and the asynchronous Pi agent fleet:

```text
┌─────────────────────────────────────────────────────────────────────────────┐
│ LAYER 1: THE TARGET REPOSITORY (e.g. superapps, mobile-core-kit, backend)  │
│  • Working Tree / Branch changes (`git diff`)                               │
│  • Specifications in `.argus/context/sfd/*.md`                              │
│  • Project-local overrides in `.argus/personas/*.md` and `config.yaml`      │
└──────────────────────────────────────┬──────────────────────────────────────┘
                                       │ Reads Diff & Config
                                       ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ LAYER 2: THE ARGUS RUST CORE (`argus` Binary)                              │
│                                                                             │
│  ┌───────────────────────┐   ┌───────────────────────┐                      │
│  │     Git Diff &        │   │    Context & SFD      │                      │
│  │   Noise Filter        │   │       Ingestion       │                      │
│  └───────────┬───────────┘   └───────────┬───────────┘                      │
│              │                           │                                  │
│              └─────────────┬─────────────┘                                  │
│                            ▼                                                │
│              ┌───────────────────────────┐                                  │
│              │ Two-Tier Persona Registry │                                  │
│              │ (Tier 1 Core + Tier 2 Repo│                                  │
│              └─────────────┬─────────────┘                                  │
│                            ▼                                                │
│              ┌───────────────────────────┐                                  │
│              │   Tokio Concurrency Pool  │                                  │
│              │  (Semaphore & Job Manager)│                                  │
│              └─────────────┬─────────────┘                                  │
│                            │ Spawns parallel processes                      │
└────────────────────────────┼────────────────────────────────────────────────┘
                             │
                             ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ LAYER 3: THE PI SUBPROCESS FLEET (Native Pi Sessions)                       │
│                                                                             │
│   ┌──────────────────────────┐     ┌──────────────────────────┐             │
│   │ Pi: sfd-clause-detective │ ... │ Pi: stock-ledger-auditor │             │
│   │ (Isolated context window)│     │ (Isolated context window)│             │
│   └─────────────┬────────────┘     └─────────────┬────────────┘             │
│                 │                                │                          │
│                 └────────────────┬───────────────┘                          │
│                                  ▼ Pipes Findings                           │
│                   ┌─────────────────────────────┐                           │
│                   │  Pi: lead-qa-synthesizer    │                           │
│                   │  (Deduplicates & Triages)   │                           │
│                   └──────────────┬──────────────┘                           │
└──────────────────────────────────┼──────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│ TERMINAL & REPORTS: Colorized P0-P2 Dashboard + Markdown + Resumable Sessions│
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. End-to-End Execution Pipeline & Data Flow

When the developer invokes `argus audit --diff --sfd docs/sfd/kulakan.md`, the system executes an 8-stage pipeline:

```text
[1. Invocation]
  Developer runs `argus audit --squad all`
         │
         ▼
[2. Git Intake & Noise Filtering]
  Argus runs `git diff origin/main` (or unstaged working tree).
  Filters out generated code: *.g.dart, *.freezed.dart, *.lock, build/
  Result: Clean, human-written diff payload (500 lines instead of 20,000).
         │
         ▼
[3. Context & SFD Assembly]
  Argus reads the active SFD Markdown file (.argus/context/sfd/kulakan.md).
  Parses sections: Acceptance Criteria, Business Rules, State Transitions.
         │
         ▼
[4. Two-Tier Persona Resolution]
  Loads 12 personas:
  - If `.argus/personas/stock-ledger-auditor.md` exists -> Use project version.
  - Else -> Use compiled-in Tier-1 base persona.
         │
         ▼
[5. Parallel Dispatch via Tokio]
  Tokio spawns up to N workers concurrently (e.g., 4 at a time via Semaphore).
  Command:
    pi -p --name "argus/<branch>/<persona>" \
          --append-system-prompt <persona.md> \
          "Evaluate this diff against your criteria: <diff + sfd>"
         │
         ▼
[6. Streaming Progress UI via Indicatif]
  Terminal displays concurrent, live spinners:
    ⠋ [sfd-clause-detective]        Auditing Section 4.2...
    ⠙ [stock-ledger-auditor]        Verifying reversibility invariants...
    ✔ [id-regulatory-sentinel]      Completed (Found 1 violation)
         │
         ▼
[7. Lead Synthesis]
  All 11 worker outputs are collected and piped to `argus/<branch>/lead-synthesizer`.
  The Synthesizer:
  - Deduplicates overlapping reports.
  - Matches against registered Behavioral Oracles (`.argus/oracles.yaml`).
  - Classifies severity: P0 (Blocker), P1 (Major), P2 (Minor).
  - Emits proposed regression test code stubs.
         │
         ▼
[8. Output & Interactive Resume]
  - Renders colorized summary table to terminal.
  - Saves Markdown report to `.argus/reports/YYYY-MM-DD-audit.md`.
  - Sessions stay alive in `~/.pi/agent/sessions/` for instant `pi -r` pairing!
```

---

## 3. Rust Internal Module Architecture (`src/`)

```text
src/
├── main.rs                      # Binary entry point & CLI dispatch
├── cli/                         # Command-line interface definitions (clap)
│   ├── mod.rs
│   ├── args.rs                  # CLI flags & subcommand parsers
│   └── commands/                # Subcommand handlers (audit, init, personas, resume)
├── config/                      # .argus/config.yaml parser & models (serde)
├── git/                         # Git diff extraction & noise filtering
├── context/                     # SFD & specification ingestion
├── persona/                     # Two-tier persona registry & discovery
├── runner/                      # Native Pi subprocess launcher & pool (tokio)
├── synthesis/                   # Finding aggregator, ranker & report generators
└── oracles/                     # .argus/oracles.yaml parser & validator
```

### Module Contracts & Type Definitions

#### 1. `git::diff` (The Noise Filter)
```rust
pub struct GitDiffOptions {
    pub base_branch: Option<String>,
    pub staged_only: bool,
    pub path_filters: Vec<String>,
}

pub struct GitDiffResult {
    pub raw_diff: String,
    pub filtered_diff: String,
    pub files_changed: Vec<PathBuf>,
    pub excluded_files: Vec<PathBuf>, // *.g.dart, *.freezed.dart, locks, etc.
}

pub async fn extract_diff(opts: &GitDiffOptions) -> Result<GitDiffResult>;
```
*Why this matters:* Large codebases contain thousands of lines of generated boilerplate (e.g. Drift `.g.dart` files). Injecting generated code exhausts LLM context windows and drowns out human-written logic. The noise filter ensures 100% of LLM reasoning bandwidth is focused on handwritten logic.

#### 2. `persona::registry` (Two-Tier Persona Engine)
```rust
pub struct Persona {
    pub name: String,
    pub title: String,
    pub squad: SquadType, // Forms, State, Sync, All
    pub model_tier: ModelTier, // Fast, Standard, Deep
    pub system_prompt: String,
    pub is_override: bool,
}

pub struct PersonaRegistry {
    personas: HashMap<String, Persona>,
}

impl PersonaRegistry {
    // 1. Loads compiled-in builtins via include_str!
    // 2. Scans <repo>/.argus/personas/*.md and overrides/merges
    pub fn load_with_project_overrides(project_root: &Path) -> Result<Self>;
    pub fn get_squad(&self, squad: &str) -> Vec<&Persona>;
}
```

#### 3. `runner::launcher` (Tokio Process Manager)
```rust
pub struct AgentExecutionPlan {
    pub session_name: String,   // "argus/DEV-AFM-AUTH-MIGRATION/stock-ledger-auditor"
    pub persona: Persona,
    pub prompt_payload: String, // Diff + SFD + instructions
}

pub struct AgentResult {
    pub persona_name: String,
    pub session_name: String,
    pub exit_code: i32,
    pub raw_output: String,
    pub duration: Duration,
}

pub async fn execute_swarm_parallel(
    plans: Vec<AgentExecutionPlan>,
    concurrency_limit: usize, // e.g. 4 concurrent pi sessions
) -> Result<Vec<AgentResult>>;
```

#### 4. `synthesis::ranker` (Findings & Severity Model)
```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Severity {
    P0Blocker, // Data loss, stock ledger imbalance, unhandled crashes on HTTP 200 OK
    P1Major,   // Bypassed form validation, illegal formats sent to backend
    P2Polish,  // Missing cosmetic labels, soft limits, suggestions
}

pub struct Finding {
    pub severity: Severity,
    pub title: String,
    pub persona: String,
    pub target_file: PathBuf,
    pub line_number: Option<usize>,
    pub oracle_id: Option<String>,
    pub explanation: String,
    pub proposed_fix: Option<String>,
}
```

---

## 4. The Concurrency & Resource Model

To balance execution speed against local CPU and API rate limits, Argus employs a **Tokio Semaphore-Bounded Worker Pool**:

```text
Total Swarm Tasks: 11 Specialists
Tokio Semaphore Capacity: 4 Concurrency Slots
─────────────────────────────────────────────────────────────────────────────
[Slot 1] ───> pi -p (sfd-clause-detective)        [Running ~15s]
[Slot 2] ───> pi -p (id-regulatory-sentinel)      [Running ~8s]  ──> DONE -> Next Task
[Slot 3] ───> pi -p (stock-ledger-auditor)        [Running ~18s]
[Slot 4] ───> pi -p (form-boundary-saboteur)      [Running ~10s] ──> DONE -> Next Task
[Queued] : cascade-dropdown-glitcher, concurrency-double-tapper, payload-pessimist...
─────────────────────────────────────────────────────────────────────────────
All 11 Done ───> [Slot 1] ───> pi -p (lead-qa-synthesizer) [Final Pass]
```

- **Controlled Concurrency:** Up to 4 subagents run simultaneously by default (configurable via `.argus/config.yaml`).
- **Memory Safety:** The Rust binary orchestrates processes consuming $< 10\text{MB}$ RAM while child `pi` processes execute in their own isolated memory spaces.
- **Fail-Safe Isolation:** If a single worker process crashes or times out, the other 10 continue uninterrupted; the Lead Synthesizer notes the partial failure and synthesizes findings from all successful runs.

---

## 5. Architectural Alignment with Field Realities

| The Real Field Failure | Architectural Prevention in Argus |
|---|---|
| **"I forgot Section 4.2 of the SFD"** | `sfd-clause-detective` reads the converted Markdown SFD directly and diffs your code against the written business requirements. |
| **"I didn't know corporate NPWP had Coretax format"** | `id-regulatory-sentinel` has the official Indonesian tax/regional regulations embedded in its persona rubric. |
| **"Deleting unpushed faktur left motoris stock deducted"** | `stock-ledger-auditor` enforces the Reversibility Law ($\Delta \text{State} = 0$), auditing all delete/cancel operations for matching return mutations. |
| **"Sync crashed on HTTP 200 with data: null"** | `payload-pessimist` scans network clients for missing default fallbacks (`const []`) and unsafe `!` unwraps. |
| **"How do I fix the issue the agent flagged?"** | Run `pi -r argus/<branch>/<persona>` and immediately chat with the agent that discovered the bug. |
