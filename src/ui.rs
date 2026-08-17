//! Presentation content: the hangman figure and the fixed message strings.
//! Owned by **Agent C**.
//!
//! Contract: `docs/contracts/ui.md`.
//!
//! Pure by design: every function is `data -> String`, no I/O. The TUI
//! (`tui.rs`) composes these strings into its frames; the fixed strings
//! here are what the e2e test asserts on. The only dependency is on
//! `game`: the `LetterOutcome` enum plus the frozen `MAX_LIVES` constant,
//! which `hangman` uses for its panic bound.

use crate::game::{LetterOutcome, MAX_LIVES};

/// The seven hangman frames, indexed by lives *lost* (0 = empty gallows,
/// 6 = complete figure).
const FRAMES: [&str; 7] = [
    "  +----+\n  |    |\n  |\n  |",
    "  +----+\n  |    |\n  |    O\n  |",
    "  +----+\n  |    |\n  |    O\n  |    |",
    "  +----+\n  |    |\n  |    O\n  |   /|\n  |",
    "  +----+\n  |    |\n  |    O\n  |   /|\\\n  |   /",
    "  +----+\n  |    |\n  |    O\n  |   /|\\\n  |   / \\",
    "  +----+\n  |    |\n  |    O\n  |   /|\\\n  |   / \\\n  ===========",
];

/// Hangman figure for the current life count.
///
/// `lives_left` is `0..=game::MAX_LIVES`. `MAX_LIVES` shows the empty
/// gallows; each lost life adds one stage (head, body, arms, legs).
/// Exactly 7 distinct frames. Panics if `lives_left > MAX_LIVES`.
pub fn hangman(lives_left: usize) -> String {
    assert!(
        lives_left <= MAX_LIVES,
        "lives_left is {lives_left}, expected at most {MAX_LIVES}"
    );
    FRAMES[MAX_LIVES - lives_left].to_string()
}

/// One-line feedback for a guess. Fixed strings (the e2e test relies on
/// them); `letter` is lowercase:
/// - `Correct`:   `'b' is in the word!`
/// - `Duplicate`: `You already guessed 'b'.`
/// - `Wrong`:     `'b' is not in the word.`
pub fn feedback(outcome: LetterOutcome, letter: char) -> String {
    match outcome {
        LetterOutcome::Correct => format!("'{letter}' is in the word!"),
        LetterOutcome::Duplicate => format!("You already guessed '{letter}'."),
        LetterOutcome::Wrong => format!("'{letter}' is not in the word."),
    }
}

/// End-of-game banner. Fixed lines (the e2e test relies on them);
/// `answer` is already uppercased, letters spaced in the display:
/// - win:  `You win!` then `The word was: C A T`
/// - loss: `You lose.` then `The word was: C A T`
pub fn ending(won: bool, answer: &str) -> String {
    let spaced = answer
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join(" ");
    if won {
        format!("You win!\nThe word was: {spaced}")
    } else {
        format!("You lose.\nThe word was: {spaced}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_seven_frames_are_distinct() {
        let frames: Vec<String> = (0..=6).map(hangman).collect();
        assert_eq!(frames.len(), 7);
        for (i, first) in frames.iter().enumerate() {
            for second in frames.iter().skip(i + 1) {
                assert_ne!(first, second, "frames {i} and {second:?} collide");
            }
        }
    }

    #[test]
    #[should_panic]
    fn hangman_above_max_lives_panics() {
        hangman(7);
    }

    #[test]
    fn feedback_exact_strings() {
        assert_eq!(feedback(LetterOutcome::Correct, 'b'), "'b' is in the word!");
        assert_eq!(
            feedback(LetterOutcome::Duplicate, 'b'),
            "You already guessed 'b'."
        );
        assert_eq!(
            feedback(LetterOutcome::Wrong, 'b'),
            "'b' is not in the word."
        );
    }

    #[test]
    fn ending_win_lines() {
        let text = ending(true, "CAT");
        assert!(text.contains("You win!"));
        assert!(text.contains("The word was: C A T"));
    }

    #[test]
    fn ending_loss_lines() {
        let text = ending(false, "CAT");
        assert!(text.contains("You lose."));
        assert!(text.contains("The word was: C A T"));
    }
}
