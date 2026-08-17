# Contract — `src/ui.rs` (Agent C)

**Owns:** `src/ui.rs`.
**May read besides its own file:** `docs/contracts/game.md` only.

## Responsibility
Rendering (pure: data → `String`) plus the stdin helpers. Data-driven by
design: render functions take plain values, never a live `GameState`, so
this module is fully testable without any other module being finished.
Depends on `game::LetterOutcome` (one enum) only.

## Frozen public API

```rust
pub fn hangman(lives_left: usize) -> String;
pub fn board(lives_left: usize, display: &str, guessed: &[char]) -> String;
pub fn feedback(outcome: LetterOutcome, letter: char) -> String;
pub fn ending(won: bool, answer: &str) -> String;
pub fn print(text: &str);          // already scaffolded
pub fn read_line(prompt: &str) -> io::Result<String>;
pub fn confirm(prompt: &str) -> io::Result<bool>;
```

Signatures are frozen. If one must change, log a change request under
"Change requests" in `PLAN.md` and work around it for now.

## Behavior
- `hangman`: exactly 7 distinct frames, index `game::MAX_LIVES - lives_left`
  (0 = empty gallows … 6 = complete figure). Panics if
  `lives_left > MAX_LIVES`.
- `board`: must contain `display` verbatim and each letter of `guessed`.
  Layout is your choice; snapshot tests pin it.
- `feedback`: one line, fixed strings (the e2e test relies on them),
  `letter` lowercase:
  - `Correct`:   `'b' is in the word!`
  - `Duplicate`: `You already guessed 'b'.`
  - `Wrong`:     `'b' is not in the word.`
- `ending`: fixed lines (e2e relies on them); `answer` is already
  uppercased; letters spaced in the display:
  - win:  `You win!` then `The word was: C A T`
  - loss: `You lose.` then `The word was: C A T`
- `read_line`: print the prompt (no newline), read one line, trim it;
  `Err(UnexpectedEof)` at end of input.
- `confirm`: re-ask until the answer is `y|yes|n|no` (case-insensitive);
  anything else (including empty) prints a hint and re-asks.

## Implementation notes (no mutable bindings)

```rust
pub fn read_line(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    let line = std::io::stdin()
        .lines()
        .next()
        .transpose()?
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "end of input"))?;
    Ok(line.trim().to_string())
}
```

`stdin().lines().next()` works on an owned temporary — no binding to
hold. `confirm` is a `loop` over `read_line` + `match` — a loop returns
values, so no binding is needed there either.

## Required tests (`#[cfg(test)] mod tests` in this file)
1. All 7 frames distinct: `hangman(0..=6)` pairwise unequal.
2. `hangman(7)` panics (`#[should_panic]`).
3. `board(3, "R _ S T _", &['a', 'd'])` contains `"R _ S T _"` and both
   `a` and `d`.
4. `feedback` returns the exact fixed strings for all three variants.
5. `ending(true, "CAT")` contains `You win!` and `The word was: C A T`;
   `ending(false, "CAT")` contains `You lose.`.
(`read_line`/`confirm` are covered by Agent D's e2e test.)

## Done when
- `cargo test ui` green
- `cargo clippy --all-targets -- -D warnings` clean
- `cargo fmt --check` clean
- `mut_budget` test green
