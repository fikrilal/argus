# Testing Strategy

This document outlines the testing conventions and patterns for **Argus**.

---

## 1. Testing Pyramid for a Rust CLI

```text
       ▲
      / \
     /CLI\        Integration Tests (`tests/cli_tests.rs`)
    /-----\       (End-to-end command execution using `assert_cmd`)
   /Functional\   Component Integration Tests (`tests/*`)
  /------------\  (Simulated Pi processes, Git repo mocks via `tempfile`)
 /  Unit Tests  \ In-tree Unit Tests (`src/**/tests.rs` or `#[cfg(test)]`)
/----------------\ (Pure logic: diff filtering, frontmatter parsing, ranker)
```

---

## 2. In-Tree Unit Tests (`src/`)

Unit tests reside directly beside the implementation code using Rust's standard `#[cfg(test)] mod tests { ... }` convention.

### Requirements:
- Pure functions (parsers, diff sanitizers, frontmatter extractors, severity rankers) must have near-100% test coverage.
- Test boundary conditions: empty inputs, unicode strings, malformed YAML frontmatter, corrupt diff chunks.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_valid_persona_frontmatter() {
        let raw = "---\nname: test-agent\nsquad: forms\n---\nHello agent";
        let parsed = parse_frontmatter(raw).expect("should parse");
        assert_eq!(parsed.name, "test-agent");
    }
}
```

---

## 3. Integration & CLI Tests (`tests/`)

Integration tests run against the compiled `argus` binary using `assert_cmd` and `tempfile`.

### Requirements:
- Verify CLI flag combinations (`--help`, `--version`, `--squad`, `--staged`).
- Verify `argus init` correctly generates files on disk in an isolated temporary directory without touching the user's real environment.
- Verify exit codes: `0` for clean audits, `1` for blocker/major findings, `2` for CLI usage errors.

---

## 4. Mocking External Processes (`git` and `pi`)

Argus interacts with external system binaries:
1. `git`: Integration tests initialize temporary Git repositories using `tempfile::tempdir()` and real `git init` commands to test diff extraction with true fidelity.
2. `pi`: Subprocess launchers support dependency-injected command runners or environment overrides (e.g. `ARGUS_PI_BIN`) so tests can execute dummy echo scripts rather than requiring real Pi LLM calls during CI.
