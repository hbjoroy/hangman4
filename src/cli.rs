//! The REPL glue: input, the game core, and rendering. Owned by **Agent D**.
//!
//! Contract: `docs/contracts/cli.md`.
//!
//! No mutable bindings anywhere: the round is a recursion over immutable
//! states (at most 26 frames deep), not a loop over a changing binding.
//! The `HANGMAN_WORD` environment variable is a deterministic test seam
//! for the e2e test.

use std::env;
use std::io;

use crate::game::GameState;
use crate::ui::{self, confirm, read_line};
use crate::words::Difficulty;

/// Run the game until the player quits. `Ok(())` on a clean exit.
pub fn run() -> io::Result<()> {
    ui::print("=== HANGMAN 4 ===");
    loop {
        let difficulty = pick_difficulty()?;
        let first = round_word(difficulty);
        let final_state = round(first)?;
        ui::print(&ui::ending(final_state.is_won(), final_state.answer()));
        if !confirm("Play again? (y/n) ")? {
            break;
        }
    }
    ui::print("Goodbye!");
    Ok(())
}

/// Difficulty menu. Accepts `""` (the default), `"1"`, `"2"` or `"3"`;
/// anything else prints a hint and re-asks.
fn pick_difficulty() -> io::Result<Difficulty> {
    ui::print("1) Easy   2) Medium   3) Hard");
    loop {
        let choice = read_line("Pick [1]: ")?;
        match choice.as_str() {
            "" | "1" => return Ok(Difficulty::Easy),
            "2" => return Ok(Difficulty::Medium),
            "3" => return Ok(Difficulty::Hard),
            _ => ui::print("Please type 1, 2 or 3."),
        }
    }
}

/// Word for the next round. If the `HANGMAN_WORD` env var is set and valid
/// (1..=26 a–z letters, either case) it wins — the deterministic e2e seam;
/// otherwise a random word from the difficulty pool.
fn round_word(difficulty: Difficulty) -> GameState {
    match env::var("HANGMAN_WORD") {
        Ok(word) if is_valid_word(&word) => GameState::new(&word),
        _ => GameState::random(difficulty),
    }
}

fn is_valid_word(word: &str) -> bool {
    !word.is_empty() && word.chars().count() <= 26 && word.chars().all(|c| c.is_ascii_alphabetic())
}

/// One round: render the board, read a guess, render the feedback, and
/// recurse until the game is over. At most 26 accepted guesses deep.
fn round(state: GameState) -> io::Result<GameState> {
    ui::print(&ui::board(
        state.lives_left(),
        &state.display(),
        &state.guessed_letters(),
    ));
    let c = read_letter()?;
    let (outcome, next) = state.guess(c);
    ui::print(&ui::feedback(outcome, c));
    if next.is_over() {
        Ok(next)
    } else {
        round(next)
    }
}

/// Read exactly one a–z letter (either case), returning lowercase;
/// anything else prints a hint and re-asks.
fn read_letter() -> io::Result<char> {
    loop {
        let line = read_line("Guess a letter: ")?;
        let chars: Vec<char> = line.chars().collect();
        if chars.len() == 1 && chars[0].is_ascii_alphabetic() {
            return Ok(chars[0].to_ascii_lowercase());
        }
        ui::print("Please type a single letter a-z.");
    }
}
