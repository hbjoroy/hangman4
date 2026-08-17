# Hangman 4 — build plan

A terminal hangman in idiomatic Rust, built under two hard constraints:

1. **Zero mutable bindings** in `src/`. The core is a pure state machine;
   the REPL is a recursion over immutable values. The policy is
   machine-enforced by `tests/mut_budget.rs`.
2. **Work divisible among agents** without anyone holding the whole
   codebase in mind: one module = one agent = one file, and all
   interfaces are frozen as short contract docs.

## Architecture

```
words ──▶ game ──▶ ui ──▶ cli ──▶ main
(data)  (core) (render) (glue)  (5 lines)
```

- `words` — word pools + random index pick. No dependencies.
- `game` — pure, immutable state machine. Depends on `words`.
- `ui` — rendering (pure `data → String`) + stdin helpers. Depends on
  `game` for one enum only (`LetterOutcome`).
- `cli` — REPL glue. Depends on all of the above.
- `main` — five-line entry point. Already written.

Dependency rule: arrows point one way only. A module never imports a
module to its right.

### How mutable bindings are avoided (mechanisms, not intentions)

| Naive approach                  | What this project does instead                     |
|---------------------------------|----------------------------------------------------|
| `let mut state` + loop          | `round(state) -> io::Result<GameState>` recursion (≤ 26 deep) |
| `BTreeSet` + `insert`           | `u32` bitmask of the 26 letters; next mask by value |
| `let mut rng`                   | `rand::thread_rng().gen_range(..)` on an owned temp |
| `let mut buf; read_line(..)`    | `stdin().lines().next().transpose()?` on a temp    |
| building `String` step by step  | `chars().map(..).collect()` / `join(" ")`          |

Enforcement: `tests/mut_budget.rs` fails `cargo test` if the identifier
token `mut` appears as a standalone word in any `src/*.rs`. ("Mutable"
inside longer identifiers is fine; the bare token is not.)

## Modules and agents

| Agent | Owns                        | Contract                  | May read besides its own file     |
|-------|-----------------------------|---------------------------|-----------------------------------|
| A     | `src/words.rs`              | `docs/contracts/words.md` | —                                 |
| B     | `src/game.rs`               | `docs/contracts/game.md`  | `docs/contracts/words.md`         |
| C     | `src/ui.rs`                 | `docs/contracts/ui.md`    | `docs/contracts/game.md`          |
| D     | `src/cli.rs`, `tests/e2e.rs`| `docs/contracts/cli.md`   | the three contracts above         |

Reading rule: an agent reads *its own file + its contract + its direct
dependency's contract*. It never reads other modules' source. Contracts
are < 120 lines and each module is capped at ~250 lines, so one agent's
entire context is a few tens of KB.

Frozen-API rule: contract signatures are frozen. If an agent believes one
must change, it implements around it and logs a change request below;
phase 3 merges requests.

## Phases

- **Phase 0 (done — this scaffold)**: `Cargo.toml`, `main.rs`, stubs of
  every public API with `todo!()` bodies, the four contracts, the
  `mut_budget` test, CI. `cargo build && cargo test` is green at commit
  one, so every agent starts from a verified baseline.
- **Phase 1 (A, B, C in parallel)**: each implements its module plus the
  unit tests listed in its contract. Gate: `cargo test <module>` green.
  - B's tests never call `GameState::random` (deterministic via
    `GameState::new`), so B does not wait for A's data.
  - C's functions take plain data, so C does not wait for B either.
- **Phase 2 (D)**: the REPL + the e2e test. The e2e uses the
  `HANGMAN_WORD` env seam, so it works before A's word data lands.
- **Phase 3 (integration, one agent or a human)**: remove the scaffold
  `#![allow(dead_code)]` from `main.rs`, resolve change requests, run
  the full CI (fmt + clippy + tests), polish the README.

## Definition of done (per module)

- `cargo test <module>` green (tests live in `#[cfg(test)]` in the file)
- `cargo clippy --all-targets -- -D warnings` clean
- `cargo fmt --check` clean
- `mut_budget` test green
- frozen signatures unchanged

## Scaling notes (where size growth goes)

- More words: `words.rs` is pure data; split into `words/easy.rs` etc.
  behind the same contract without touching any other module.
- Bigger screens / TUI: swap `ui` for a different implementation behind
  the same data-driven signatures; `cli` is untouched.
- Multiplayer: a new module that passes `GameState` values across a
  channel; the core stays pure and immutable.
- New word sources (files, API): add functions to the `words` contract;
  `game` is untouched.

The rule of thumb: growth happens *inside* a module or as a *new* module
downstream; frozen contracts are the only cross-agent surface, and they
stay small.

## Change requests

(Phase 1–2 agents log contract change requests here.)

- none yet
