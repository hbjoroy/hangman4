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
        let upper = word.to_ascii_uppercase();
        assert!(!upper.is_empty(), "word must not be empty");
        assert!(
            upper.chars().count() <= 26,
            "word must be at most 26 letters, got {} in {word:?}",
            upper.chars().count()
        );
        assert!(
            upper.chars().all(|c| c.is_ascii_alphabetic()),
            "word must contain only a–z letters, got {word:?}",
        );
        Self {
            word: upper,
            mask: 0,
            lives_lost: 0,
        }
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
        let c = letter.to_ascii_lowercase();
        assert!(
            c.is_ascii_lowercase(),
            "guess expects a letter a–z, got {letter:?}",
        );
        let bit = 1u32 << (c as u32 - b'a' as u32);
        if (self.mask & bit) != 0 {
            return (
                LetterOutcome::Duplicate,
                Self {
                    word: self.word.clone(),
                    mask: self.mask,
                    lives_lost: self.lives_lost,
                },
            );
        }
        let correct = self.word.contains(c.to_ascii_uppercase());
        (
            if correct {
                LetterOutcome::Correct
            } else {
                LetterOutcome::Wrong
            },
            Self {
                word: self.word.clone(),
                mask: self.mask | bit,
                lives_lost: self.lives_lost + usize::from(!correct),
            },
        )
    }

    /// Lives remaining: `MAX_LIVES - lives_lost`.
    pub fn lives_left(&self) -> usize {
        MAX_LIVES - self.lives_lost
    }

    /// The word as displayed mid-game: guessed letters revealed, hidden
    /// letters as `_`, joined with single spaces. Example: after
    /// guessing `s` in `"RUST"`, `display == "_ _ S _"`.
    pub fn display(&self) -> String {
        self.word
            .chars()
            .map(|ch| {
                let bit = 1u32 << (ch as u32 - b'A' as u32);
                if (self.mask & bit) != 0 {
                    ch.to_string()
                } else {
                    "_".to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// All letters guessed so far, sorted, lowercase.
    pub fn guessed_letters(&self) -> Vec<char> {
        (0..26)
            .filter(|i| (self.mask >> i) & 1 != 0)
            .map(|i| (i as u8 + b'a') as char)
            .collect()
    }

    /// True when the word is fully revealed or all lives are lost.
    pub fn is_over(&self) -> bool {
        self.is_won() || self.lives_lost == MAX_LIVES
    }

    /// True iff the game is over and the player won.
    pub fn is_won(&self) -> bool {
        self.word
            .chars()
            .all(|ch| (self.mask >> (ch as u32 - b'A' as u32)) & 1 != 0)
    }

    /// The secret word (uppercased). For display after game over.
    pub fn answer(&self) -> &str {
        &self.word
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_uppercases() {
        assert_eq!(GameState::new("cat").answer(), "CAT");
        assert_eq!(GameState::new("MiXeD").answer(), "MIXED");
    }

    #[test]
    #[should_panic]
    fn new_rejects_empty() {
        GameState::new("");
    }

    #[test]
    #[should_panic]
    fn new_rejects_non_letter() {
        GameState::new("a b");
    }

    #[test]
    #[should_panic]
    fn new_rejects_27_letters() {
        GameState::new("abcdefghijklmnopqrstuvwxyzx");
    }

    #[test]
    fn win_path_on_cat() {
        let (o1, s1) = GameState::new("CAT").guess('c');
        assert_eq!(o1, LetterOutcome::Correct);
        let (o2, s2) = s1.guess('a');
        assert_eq!(o2, LetterOutcome::Correct);
        let (o3, s3) = s2.guess('t');
        assert_eq!(o3, LetterOutcome::Correct);
        assert!(s3.is_won());
        assert!(s3.is_over());
        assert_eq!(s3.display(), "C A T");
        assert_eq!(s3.lives_left(), MAX_LIVES);
    }

    #[test]
    fn loss_path_on_pony() {
        let (o1, s1) = GameState::new("PONY").guess('a');
        assert_eq!(o1, LetterOutcome::Wrong);
        let (o2, s2) = s1.guess('b');
        assert_eq!(o2, LetterOutcome::Wrong);
        let (o3, s3) = s2.guess('c');
        assert_eq!(o3, LetterOutcome::Wrong);
        let (o4, s4) = s3.guess('d');
        assert_eq!(o4, LetterOutcome::Wrong);
        let (o5, s5) = s4.guess('e');
        assert_eq!(o5, LetterOutcome::Wrong);
        let (o6, s6) = s5.guess('f');
        assert_eq!(o6, LetterOutcome::Wrong);
        assert!(s6.is_over());
        assert!(!s6.is_won());
        assert_eq!(s6.lives_left(), 0);
    }

    #[test]
    fn duplicate_guess_keeps_the_state() {
        let (o1, s1) = GameState::new("CAT").guess('x');
        assert_eq!(o1, LetterOutcome::Wrong);
        let (o2, s2) = s1.guess('x');
        assert_eq!(o2, LetterOutcome::Duplicate);
        assert_eq!(s2.lives_left(), MAX_LIVES - 1);
        assert_eq!(s2.display(), s1.display());
        assert_eq!(s2.guessed_letters(), s1.guessed_letters());
    }

    #[test]
    fn partial_display_on_rust() {
        // Contract test 5 pins `display == "R _ S _"`, which under the
        // semantics section requires both `r` and `s` to be guessed
        // (see the change request in PLAN.md).
        let (o1, s1) = GameState::new("RUST").guess('r');
        assert_eq!(o1, LetterOutcome::Correct);
        let (o2, s) = s1.guess('s');
        assert_eq!(o2, LetterOutcome::Correct);
        assert_eq!(s.display(), "R _ S _");
        // And with only `s` guessed, `R` stays hidden.
        let (_o, s_only) = GameState::new("RUST").guess('s');
        assert_eq!(s_only.display(), "_ _ S _");
    }

    #[test]
    fn guessed_letters_are_sorted_lowercase() {
        let (_o1, s1) = GameState::new("CAT").guess('T');
        let (_o2, s2) = s1.guess('c');
        assert_eq!(s2.guessed_letters(), vec!['c', 't']);
        for c in s2.guessed_letters() {
            assert!(c.is_ascii_lowercase());
        }
    }

    #[test]
    fn fresh_states_are_deterministic() {
        let a = GameState::new("CAT");
        let b = GameState::new("CAT");
        assert_eq!(a.display(), b.display());
        assert_eq!(a.lives_left(), b.lives_left());
        assert_eq!(a.guessed_letters(), b.guessed_letters());
        let (oa, sa) = a.guess('c');
        let (ob, sb) = b.guess('c');
        assert_eq!(oa, ob);
        assert_eq!(sa.display(), sb.display());
        assert_eq!(sa.lives_left(), sb.lives_left());
    }
}
