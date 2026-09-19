# Design Principles

Argus treats complexity as the primary cost of software design. Correct code is not sufficient when its structure makes future changes difficult or unsafe.

These principles apply to generated code, handwritten changes, design proposals, and code review.

---

## 1. Optimize For Lower Complexity

- Prefer the design that leaves fewer concepts, dependencies, states, and rules for future contributors to understand.
- Evaluate complexity at the system level, not only by local line count.
- Do not trade a small local simplification for wider coupling or hidden global complexity.

## 2. Prefer Deep Modules

- Prefer small, stable interfaces that hide substantial implementation detail.
- A new interface must remove meaningful complexity from its callers.
- Avoid thin wrappers that rename an underlying API without improving the abstraction.

## 3. Hide Information

- Keep Git diff parsing, persona resolution, subprocess execution, and Markdown formatting behind their owning boundaries.
- Expose domain meaning rather than implementation mechanics.
- Do not make callers coordinate steps that a module can own internally.

## 4. Design Away Special Cases

- Repeated conditionals, mode flags, and special cases are design signals.
- Prefer a general invariant or abstraction when it removes real special cases.
- Do not create an abstraction solely to make unlike behavior appear uniform.

## 5. Separate Interface From Implementation

- Public contracts describe what a module provides, not how it provides it.
- Implementation details may change without forcing unrelated callers to change.
- Tests should primarily verify observable contracts and important invariants.

## 6. Apply KISS, YAGNI, And DRY Pragmatically

- **KISS:** Choose the simplest design that preserves required boundaries and safety invariants.
- **YAGNI:** Do not add extension points, configuration, indirection, or generalized behavior without a current requirement.
- **DRY:** Remove duplicated knowledge and policy, not merely similar-looking lines. Duplication is cheaper than a premature abstraction that couples unrelated behavior.

## 7. Review Standard

Reject or redesign changes that:
- Add abstractions for hypothetical reuse.
- Expose implementation details across an architectural boundary.
- Add boolean flags that create behavioral modes instead of distinct concepts.
- Introduce `.unwrap()` or `.expect()` in non-test paths.
- Add generic file or module names (`helper`, `manager`, `utils`).
