//! Core game logic — a pure, immutable state machine. Owned by **Agent B**.
//!
//! Contract: `docs/contracts/game.md`.
//!
//! The heart of this project's immutability policy: `GameState` is a value
//! and every transition takes `&self` and returns a new state, so the
//! REPL (see `cli::round`) can be written without mutable bindings.

use crate::words::{Difficulty, random_index};

/// Maximum number of wrong guesses allowed.
pub const MAX_LIVES: usize = 6;

/// Outcome of a single letter guess.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LetterOutcome {
    /// The letter occurs in the word (the game may now be finished).
    Correct,
    /// The letter was already guessed before. No life is lost.
    Duplicate,
    /// The letter does not occur in the word. One life is lost.
    Wrong,
}

/// Immutable hangman game state.
///
/// All fields are private; the only ways to build a valid state are
/// [`GameState::new`] and [`GameState::random`].
#[derive(Debug, Clone)]
pub struct GameState {
    /// Secret word, uppercased a–z, 1..=26 letters.
    word: String,
    /// Bit `i` set iff the lowercase letter `i + 'a'` has been guessed.
    mask: u32,
    /// Wrong guesses so far. Invariant: `0..=MAX_LIVES`.
    lives_lost: usize,
}

impl GameState {
    /// Start a new game with `word`.
    ///
    /// The word is normalized to uppercase. Panics if it is empty, longer
    /// than 26 letters, or contains a non-a–z character.
    pub fn new(word: &str) -> Self {
        todo!("Agent B — new({word:?}) — see docs/contracts/game.md")
    }

    /// Start a new game with a random word from the given difficulty pool.
    pub fn random(difficulty: Difficulty) -> Self {
        let pool = difficulty.pool();
        Self::new(pool[random_index(pool.len())])
    }

    /// Guess `letter` (a–z, either case).
    ///
    /// Returns the outcome and the next state. Panics if `letter` is not
    /// a–z (callers validate input via `cli::read_letter`). A duplicate
    /// guess returns the state unchanged.
    pub fn guess(&self, letter: char) -> (LetterOutcome, Self) {
        todo!("Agent B — guess({letter:?}) — see docs/contracts/game.md")
    }

    /// Lives remaining: `MAX_LIVES - lives_lost`.
    pub fn lives_left(&self) -> usize {
        todo!("Agent B — see docs/contracts/game.md")
    }

    /// The word as displayed mid-game: guessed letters revealed, hidden
    /// letters as `_`, joined with single spaces. Example: `"R _ S T _"`.
    pub fn display(&self) -> String {
        todo!("Agent B — see docs/contracts/game.md")
    }

    /// All letters guessed so far, sorted, lowercase.
    pub fn guessed_letters(&self) -> Vec<char> {
        todo!("Agent B — see docs/contracts/game.md")
    }

    /// True when the word is fully revealed or all lives are lost.
    pub fn is_over(&self) -> bool {
        todo!("Agent B — see docs/contracts/game.md")
    }

    /// True iff the game is over and the player won.
    pub fn is_won(&self) -> bool {
        todo!("Agent B — see docs/contracts/game.md")
    }

    /// The secret word (uppercased). For display after game over.
    pub fn answer(&self) -> &str {
        todo!("Agent B — see docs/contracts/game.md")
    }
}
