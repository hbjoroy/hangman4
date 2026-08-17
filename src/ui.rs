//! Terminal rendering and input. Owned by **Agent C**.
//!
//! Contract: `docs/contracts/ui.md`.
//!
//! Data-driven by design: the render functions take plain values
//! (`usize`, `&str`, `&[char]`), never a live `GameState`, so this module
//! can be snapshot-tested without any other module being finished. The
//! only dependency on `game` is the `LetterOutcome` enum.

use std::io;

use crate::game::LetterOutcome;

/// Hangman figure for the current life count.
///
/// `lives_left` is `0..=game::MAX_LIVES`. `MAX_LIVES` shows the empty
/// gallows; each lost life adds one stage (head, body, arms, legs).
/// Exactly 7 distinct frames. Panics if `lives_left > MAX_LIVES`.
pub fn hangman(lives_left: usize) -> String {
    todo!("Agent C — hangman({lives_left}) — see docs/contracts/ui.md")
}

/// Full mid-game board: figure, word, guessed letters, remaining lives.
///
/// Must contain `display` verbatim and each letter of `guessed` (the e2e
/// test relies on that). Layout is Agent C's choice; snapshot tests pin it.
pub fn board(lives_left: usize, display: &str, guessed: &[char]) -> String {
    todo!("Agent C — board({lives_left}, {display:?}, {guessed:?}) — see docs/contracts/ui.md")
}

/// One-line feedback for a guess. Fixed strings (the e2e test relies on
/// them); `letter` is lowercase:
/// - `Correct`:   `'b' is in the word!`
/// - `Duplicate`: `You already guessed 'b'.`
/// - `Wrong`:     `'b' is not in the word.`
pub fn feedback(outcome: LetterOutcome, letter: char) -> String {
    todo!("Agent C — feedback({outcome:?}, {letter:?}) — see docs/contracts/ui.md")
}

/// End-of-game banner. Fixed lines (the e2e test relies on them);
/// `answer` is already uppercased, letters spaced in the display:
/// - win:  `You win!` then `The word was: C A T`
/// - loss: `You lose.` then `The word was: C A T`
pub fn ending(won: bool, answer: &str) -> String {
    todo!("Agent C — ending({won}, {answer:?}) — see docs/contracts/ui.md")
}

/// Print `text` (plus a newline) to stdout.
pub fn print(text: &str) {
    println!("{text}");
}

/// Print `prompt` to stdout (no newline), then read one line from stdin,
/// trimmed. `Err(UnexpectedEof)` on end of input.
pub fn read_line(prompt: &str) -> io::Result<String> {
    todo!("Agent C — read_line({prompt:?}) — see docs/contracts/ui.md")
}

/// Ask a yes/no question, re-asking until the answer is `y`/`n`
/// (case-insensitive; `yes`/`no` accepted too).
pub fn confirm(prompt: &str) -> io::Result<bool> {
    todo!("Agent C — confirm({prompt:?}) — see docs/contracts/ui.md")
}
