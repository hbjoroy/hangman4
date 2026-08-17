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
words ──▶ game ──▶ ui ──▶ tui ──▶ main
(data)  (core) (content)(front-end) (5 lines)
```

- `words` — word pools + random index pick. No dependencies.
- `game` — pure, immutable state machine. Depends on `words`.
- `ui` — pure presentation content (figure frames, fixed message
  strings, `data → String`). Depends on `game` for one enum only
  (`LetterOutcome`).
- `tui` — the game's only front-end (the REPL), split into three files
  (`mod.rs` state machine, `frames.rs` layout/rendering, `input.rs`
  token I/O). TTY: crossterm raw mode + colored in-place frames;
  headless (piped stdio, e.g. the e2e): line-based input + plain-text
  frames with the same fixed strings. Depends on `game`, `ui`, `words`,
  and the `crossterm` crate.
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
| D     | `tests/e2e.rs`              | `docs/contracts/cli.md` (retired; now `tui.md`) | the three contracts above |
| E     | `src/tui/`                  | `docs/contracts/tui.md`   | `game`, `ui`, `words` contracts   |

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
- **Phase 4 (TUI iteration, done)**: introduce `crossterm` and replace
  the stdio REPL with `src/tui/` as the only front-end (TTY: raw mode
  + colored in-place frames; headless: line-based input, same fixed
  strings — so the e2e is unchanged). `src/cli.rs` retired; `ui` reduced
  to pure content. Gates unchanged: fmt, clippy `-D warnings`, full
  `cargo test` (incl. e2e + `mut_budget`).

## Definition of done (per module)

- `cargo test <module>` green (tests live in `#[cfg(test)]` in the file)
- `cargo clippy --all-targets -- -D warnings` clean
- `cargo fmt --check` clean
- `mut_budget` test green
- frozen signatures unchanged

## Scaling notes (where size growth goes)

- More words: `words.rs` is pure data; split into `words/easy.rs` etc.
  behind the same contract without touching any other module.
- Bigger screens / TUI: **done in Phase 4** — `crossterm` behind
  `tui`'s private API. Crossterm (not ratatui) because ratatui's
  `Terminal` buffer needs a mutable binding while crossterm's `execute!`
  + owned `Event` values do not; `tui` keeps the zero-mutable policy
  machine-enforced. A richer renderer (widgets, input boxes) is a
  private change inside `tui` behind the same `run()` contract.
- Multiplayer: a new module that passes `GameState` values across a
  channel; the core stays pure and immutable.
- New word sources (files, API): add functions to the `words` contract;
  `game` is untouched.

The rule of thumb: growth happens *inside* a module or as a *new* module
downstream; frozen contracts are the only cross-agent surface, and they
stay small.

## Change requests

(Phase 1–2 agents log contract change requests here.)

- **(A, words.md)** The implementation hint `rand::thread_rng().gen_range(..)`
  predates rand 0.9.5 (locked by `Cargo.lock`): both names are renamed
  (`rand::rng()` / `Rng::random_range(..)`) and the old ones emit deprecation
  warnings, which the `-D warnings` CI gate rejects. `random_index` is
  implemented with the new names; the frozen signature is unchanged. Suggest
  the contract be updated to the current rand API in the next pass.
- **(B, game.md)** Test 5 pins `display == "R _ S _"` for `"RUST"` "after
  guessing `s`", but the semantics section (revealed letters in place, `_`
  for hidden) implies `R` is revealed only if `r` was guessed. The test is
  implemented with the guess sequence `{r, s}`, which yields exactly the
  pinned string, plus an extra assertion that guessing only `s` gives
  `"_ _ S _"`. No signature changed. Suggest the contract's guess list be
  corrected to `r`, `s`.
- **(D, cli.md)** The e2e win input `1\nb\na\nc\nn\n` can never win with
  `HANGMAN_WORD=CAT` (the letter `t` is never guessed; `n` is consumed as a
  letter guess and the run ends in an EOF error, exit 1). The e2e uses
  `1\nb\na\nc\nt\nn\n`, which satisfies every pinned expected output line
  (`'b' is not in the word.`, `'a' is in the word!`, `You win!`,
  `The word was: C A T`, exit 0). No signature changed. Suggest the
  contract's input be corrected to include `t`.
- **(E, new `tui.md`)** Phase 4 adds `src/tui/` as the game's only
  front-end and retires `src/cli.rs`. `crossterm 0.29` is the TUI
  library (over ratatui, whose `Terminal` buffer requires a mutable
  binding). The TUI has two transports — TTY (raw mode, alternate
  screen, colored in-place frames) and headless (one stdin line per
  event, plain frames) — so the e2e keeps its line-based input, fixed
  strings, and exit code 0 unchanged. `q` is a letter guess in-round
  (a word may contain Q); only `Esc` quits a round on the TTY
  transport.
- **(E, `cli.md`)** Retired: the REPL behaviors this contract pinned
  (`HANGMAN_WORD` seam, line input semantics, fixed strings, e2e)
  survive in `tui` / the headless transport. The file is kept as a
  short retirement note.
- **(E, `ui.md`)** The I/O surface (`board`, `print`, `read_line`,
  `confirm`) is retired — superseded by the TUI's frame renderer and
  line-based token reader. `ui` is now pure content: `hangman`,
  `feedback`, `ending` (the fixed strings the e2e asserts on).
