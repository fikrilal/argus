# Execution Plans

This directory manages structured, bounded execution plans for non-trivial engineering tasks on **Argus**.

## Structure

- `_template.md`: Canonical template for creating a new execution plan.
- `active/`: Plans currently being implemented. Exactly one active plan is the default.
- `completed/`: Historical record of delivered plans and their completion evidence.

## Guidelines

1. Copy `_template.md` to `active/YYYY-MM-DD_<task-name>.md`.
2. Define the objective, constraints, impact areas, acceptance criteria, and implementation checklist.
3. Execute tasks following the **Execute $\to$ Review $\to$ Test $\to$ Commit** cycle.
4. Record exact verification commands and outcomes in the plan.
5. Move the completed plan to `completed/` when finished.
