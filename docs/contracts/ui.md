# Contract — `src/ui.rs` (Agent C)

**Owns:** `src/ui.rs`.
**May read besides its own file:** `docs/contracts/game.md` only.

## Responsibility
Pure presentation content: the hangman figure and the fixed message
strings, as `data -> String` functions. No I/O. The TUI
(`src/tui/`) composes these strings into its frames; the fixed strings
here are what the e2e test asserts on.

> The TUI iteration retired the former I/O surface of this module
> (`board`, `print`, `read_line`, `confirm`) — superseded by the TUI's
> frame renderer and line-based token reader. See the change request in
> `PLAN.md`.

## Frozen public API

```rust
pub fn hangman(lives_left: usize) -> String;
pub fn feedback(outcome: LetterOutcome, letter: char) -> String;
pub fn ending(won: bool, answer: &str) -> String;
```

Signatures are frozen. If one must change, log a change request under
"Change requests" in `PLAN.md` and work around it for now.

## Behavior
- `hangman`: exactly 7 distinct frames, index `game::MAX_LIVES -
  lives_left` (0 = empty gallows … 6 = complete figure). Panics if
  `lives_left > MAX_LIVES`.
- `feedback`: one line, fixed strings (the e2e test relies on them),
  `letter` lowercase:
  - `Correct`:   `'b' is in the word!`
  - `Duplicate`: `You already guessed 'b'.`
  - `Wrong`:     `'b' is not in the word.`
- `ending`: fixed lines (e2e relies on them); `answer` is already
  uppercased; letters spaced in the display:
  - win:  `You win!` then `The word was: C A T`
  - loss: `You lose.` then `The word was: C A T`

## Required tests (`#[cfg(test)] mod tests` in this file)
1. All 7 frames distinct: `hangman(0..=6)` pairwise unequal.
2. `hangman(7)` panics (`#[should_panic]`).
3. `feedback` returns the exact fixed strings for all three variants.
4. `ending(true, "CAT")` contains `You win!` and `The word was: C A T`;
   `ending(false, "CAT")` contains `You lose.`.

## Done when
- `cargo test ui` green
- `cargo clippy --all-targets -- -D warnings` clean
- `cargo fmt --check` clean
- `mut_budget` test green
