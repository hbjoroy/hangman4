# Contract — `src/game.rs` (Agent B)

**Owns:** `src/game.rs`.
**May read besides its own file:** `docs/contracts/words.md` only.

## Responsibility
The pure core: an immutable state machine. No I/O, no `ui`, no global
state. Depends only on the frozen `words` API.

## Frozen public API

```rust
pub const MAX_LIVES: usize; // = 6

pub enum LetterOutcome { Correct, Duplicate, Wrong }

pub struct GameState { /* private fields, see scaffold */ }

impl GameState {
    pub fn new(word: &str) -> Self;
    pub fn random(difficulty: Difficulty) -> Self; // already scaffolded
    pub fn guess(&self, letter: char) -> (LetterOutcome, Self);
    pub fn lives_left(&self) -> usize;
    pub fn display(&self) -> String;
    pub fn guessed_letters(&self) -> Vec<char>;
    pub fn is_over(&self) -> bool;
    pub fn is_won(&self) -> bool;
    pub fn answer(&self) -> &str;
}
```

Signatures are frozen. If one must change, log a change request under
"Change requests" in `PLAN.md` and work around it for now.

## Semantics
- `new(word)`: normalizes to uppercase. Panics if empty, longer than 26
  letters, or contains a non-a–z character. Fresh state: nothing guessed,
  `lives_lost == 0`.
- `guess(c)`:
  - panics if `c` is not a–z (either case);
  - already guessed → `(Duplicate, state unchanged)`;
  - occurs in the word → `(Correct, state with that letter's bit set)`;
  - otherwise → `(Wrong, bit set and one life lost)`.
- `is_won`: every letter of the word has been guessed.
- `is_over`: `is_won || lives_lost == MAX_LIVES`.
- `lives_left`: `MAX_LIVES - lives_lost`.
- `display`: uppercased, revealed letters in place, `_` for hidden,
  joined with single spaces — e.g. `"R _ S T _"`.
- `guessed_letters`: sorted, lowercase.
- `answer`: the word, uppercased.

## Representation (frozen)
`word: String` (uppercased), `mask: u32` (bit `i` = lowercase letter
`i + 'a'` has been guessed), `lives_lost: usize`. The bitmask keeps
`guess` free of any set-insertion machinery.

## Implementation notes (no mutable bindings)
`guess` builds the next state by value:

```rust
let c = letter.to_ascii_lowercase();
if !c.is_ascii_alphabetic() {
    panic!("guess expects a letter a–z, got {letter:?}");
}
let bit = 1u32 << (c as u32 - b'a' as u32);
if self.mask & bit != 0 {
    return (
        LetterOutcome::Duplicate,
        Self {
            word: self.word.clone(),
            mask: self.mask,
            lives_lost: self.lives_lost,
        },
    );
}
let correct = self.word.contains(c);
(
    if correct { LetterOutcome::Correct } else { LetterOutcome::Wrong },
    Self {
        word: self.word.clone(),
        mask: self.mask | bit,
        lives_lost: self.lives_lost + usize::from(!correct),
    },
)
```

`display` via `self.word.chars().map(…).collect::<Vec<_>>().join(" ")`.
`guessed_letters` walks bits 0..26 and collects the set ones.

## Required tests (`#[cfg(test)] mod tests` in this file)
1. `new` uppercases; `#[should_panic]` for `""`, `"a b"`, 27 letters.
2. Win path on `"CAT"`: guess `c`, `a`, `t` — after `t`: `is_won`,
   `is_over`, `display == "C A T"`, `lives_left == MAX_LIVES`.
3. Loss path on `"PONY"`: guess `a,b,c,d,e,f` — after `f`: `is_over`,
   `!is_won`, `lives_left == 0`.
4. Duplicate on `"CAT"`: guess `x` then `x` again — second returns
   `Duplicate`; `lives_left == MAX_LIVES - 1` after both.
5. Partial display on `"RUST"` after guessing `s`: `display == "R _ S _"`.
6. `guessed_letters` sorted and lowercase.
7. Two fresh `new("CAT")` states behave identically (determinism).

## Done when
- `cargo test game` green
- `cargo clippy --all-targets -- -D warnings` clean
- `cargo fmt --check` clean
- `mut_budget` test green
