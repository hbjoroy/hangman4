# Contract — `src/cli.rs` (Agent D)

**Owns:** `src/cli.rs` and the new `tests/e2e.rs`.
**May read besides its own file:** the `words`, `game`, and `ui` contracts.

## Responsibility
The REPL glue: read input, call the game core, render through `ui`.
Public API is `run` only; the private split below is a suggestion, not
frozen.

## Frozen public API

```rust
pub fn run() -> io::Result<()>;
```

## Behavior
- `run()`:
  1. print a title line containing `HANGMAN`;
  2. loop:
     - `pick_difficulty()` — menu line `1) Easy   2) Medium   3) Hard`,
       prompt `Pick [1]: `, accepts `""|"1"|"2"|"3"`; anything else
       prints `Please type 1, 2 or 3.` and re-asks;
     - start the round via `round_word(difficulty)`;
     - `round(state)` until it is over;
     - print `ui::ending(final.is_won(), final.answer())`;
     - `ui::confirm("Play again? (y/n) ")` — break on `false`;
  3. print a goodbye line; return `Ok(())`.
  All `io::Error`s (including EOF) propagate to `main`.
- `round_word(d: Difficulty) -> GameState` (private): if the
  `HANGMAN_WORD` env var is set and valid (1..=26 a–z letters, either
  case), use `GameState::new` — this is the deterministic e2e test
  seam; otherwise `GameState::random(d)`.
- `round(state: GameState) -> io::Result<GameState>` (private,
  recursive — no mutable binding; at most 26 frames deep, one per
  accepted guess):

  ```rust
  ui::print(&ui::board(
      state.lives_left(),
      &state.display(),
      &state.guessed_letters(),
  ));
  let c = read_letter()?;
  let (outcome, next) = state.guess(c);
  ui::print(&ui::feedback(outcome, c));
  if next.is_over() { Ok(next) } else { round(next) }
  ```

- `read_letter() -> io::Result<char>` (private): a `loop` (retry count is
  unbounded, but a loop never grows the stack) until the input is exactly
  one a–z letter; prompt `Guess a letter: `; anything else prints
  `Please type a single letter a-z.` and re-asks. Returns lowercase.

## e2e test (new `tests/e2e.rs`)
Spawn `env!("CARGO_BIN_EXE_hangman4")` with `HANGMAN_WORD=CAT` in the
environment and feed lines on its stdin (use owned
`Stdio::piped()` handles — no mutable bindings needed):

1. Win: input `1\nb\na\nc\nn\n` → stdout contains
   `'b' is not in the word.`, `'a' is in the word!`, `You win!`,
   `The word was: C A T`; exit code 0.
2. Loss: input `1\nz\ny\nx\nw\nv\nu\nn\n` (six wrong guesses) → stdout
   contains `You lose.` and `The word was: C A T`; exit code 0.

## Done when
- `cargo test` (all targets, including e2e) green
- `cargo clippy --all-targets -- -D warnings` clean
- `cargo fmt --check` clean
- `mut_budget` test green
