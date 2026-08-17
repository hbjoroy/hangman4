# Contract — `src/tui/` (Agent E)

**Owns:** `src/tui/` (`mod.rs` — state machine, `frames.rs` — layout and
rendering, `input.rs` — token reading and mapping).
**May read besides its own file:** `docs/contracts/game.md`,
`docs/contracts/ui.md`, `docs/contracts/words.md`.

## Responsibility
The game's only front-end. Runs the whole session (menu, rounds, ending)
over two transports, chosen once from the stdio handles:

- **TTY** — stdin and stdout are both terminals: crossterm raw mode,
  alternate screen, hidden cursor; frames drawn in place (colored, padded
  rows + cursor moves; no clears).
- **Headless** — anything else (piped stdin/stdout, e.g. the e2e test):
  one line of stdin per input event, plain-text frames with the same
  fixed strings. This keeps the e2e's line semantics (`1`, `b`, …, `n`)
  and its exit code 0 unchanged.

Depends on `game`, `ui` (content only), `words`, and the `crossterm`
crate. Arrows point one way only.

## Frozen public API

```rust
pub fn run() -> io::Result<()>;
```

Everything else is private. Clean quit exits with `Ok(())` (process exit
code 0). Terminal teardown (show cursor, leave alternate screen, disable
raw mode) runs explicitly on every exit path once raw mode is enabled —
a guard type cannot work because its drop signature requires a
non-immutable receiver.

## Behavior
- **Input tokens**, one per read:
  - TTY: crossterm key-press events (repeats/releases ignored).
  - Headless: one trimmed line of stdin; `Err(UnexpectedEof)` at end of
    input.
- **Menu**: `1`/`2`/`3` (or a blank line = `1`) picks Easy/Medium/Hard;
  `q`/Esc quits; anything else re-renders with the hint
  `Please type 1, 2 or 3.`
- **Round**: one a–z letter (either case) is a guess — `q` included, so
  no word is unguessable; `Esc` quits a round (and the game) cleanly on
  the TTY transport; the headless transport has no mid-round quit,
  matching the retired CLI; anything else re-renders with the hint
  `Please type a single letter a-z.`
- **Ending**: `y`/`yes` plays again (back to the menu); `n`/`no`/`q`/Esc
  quits.
- **Word seam**: the `HANGMAN_WORD` env var (1..=26 a–z letters, either
  case) overrides the random pick — the deterministic e2e seam
  (inherited from the retired `cli` contract).
- **Fixed strings** come from `ui::feedback` / `ui::ending` and appear
  verbatim in both transports (the e2e asserts on them).

## Frame layout (both transports; 14 rows)
Row 0: title `HANGMAN 4`. Rows 2–7: the `ui::hangman` figure, with
`Lives: *****.` at column 12 of row 4. Row 9: `Word: <display>`.
Row 10: `Guessed: <letters, or ->`. Row 11: the last `ui::feedback`
line (carried into the next frame so it stays visible). Row 13: the
prompt, or a hint after invalid input. Menu/ending/goodbye frames use
the same rows.

## Implementation notes (no mutable bindings)
- Crossterm was chosen over ratatui: ratatui's `Terminal` buffer needs a
  mutable binding; crossterm's `execute!` + owned `Event` values keep
  this module within the policy.
- The shell is a rebind-free `while let` loop; `play_round` is a
  recursion over immutable `GameState` values (one frame per state, so
  depth tracks accepted guesses); token readers are stateless filter
  loops. No mutable bindings anywhere, machine-checked by
  `tests/mut_budget.rs`.

## Required tests (`#[cfg(test)]` in the TUI files)
1. `input.rs`: `menu_action` / `game_action` / `again_action` token
   mapping: lines and keys, case-folding, `q` as a letter guess,
   invalid input, quit.
2. `frames.rs`: the game block contains the word display and the fixed
   feedback string; the ending block contains `You win!` and
   `The word was: C A T`; every frame is exactly 14 rows.
3. `mod.rs`: the `HANGMAN_WORD` validator accepts 1..=26 a–z letters
   and rejects the rest.

## Done when
- `cargo test` green (including the e2e through the headless transport)
- `cargo clippy --all-targets -- -D warnings` clean
- `cargo fmt --check` clean
- `mut_budget` test green
