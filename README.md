# Hangman 4

A terminal hangman game in idiomatic Rust: a pure immutable game core,
a thin I/O shell, and zero mutable bindings in `src/` (machine-enforced
by `tests/mut_budget.rs`, which fails `cargo test` on a bare `mut` token).

## Run

```
cargo run
```

## Develop

See `PLAN.md` for the architecture, agent assignment, and per-module
contracts in `docs/contracts/`. The CI gate (also a local habit) is:

```
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```
