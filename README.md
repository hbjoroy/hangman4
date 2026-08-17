# Hangman 4

A terminal hangman game in idiomatic Rust: a pure immutable game core,
a TUI front-end, and zero mutable bindings in `src/` (machine-enforced
by `tests/mut_budget.rs`, which fails `cargo test` on a bare `mut`
token).

## Run

```
cargo run
```

On a terminal you get the full TUI: colored, in-place frames, raw-mode
key input. With piped stdin/stdout (e.g. `echo "1" | cargo run`) the
same TUI runs in headless mode: one input line per key, plain-text
frames — which is also how the e2e test drives the binary.

## Develop

See `PLAN.md` for the architecture, agent assignment, and per-module
contracts in `docs/contracts/`. The CI gate (also a local habit) is:

```
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```
