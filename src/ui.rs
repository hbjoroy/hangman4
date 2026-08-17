//! Terminal rendering and input. Owned by **Agent C**.
//!
//! Contract: `docs/contracts/ui.md`.
//!
//! Data-driven by design: the render functions take plain values
//! (`usize`, `&str`, `&[char]`), never a live `GameState`, so this module
//! can be snapshot-tested without any other module being finished. The
//! only dependency on `game` is the `LetterOutcome` enum (plus the frozen
//! `MAX_LIVES` constant, which `hangman` uses for its panic bound).

use std::io::{self, Write};

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

/// Full mid-game board: figure, word, guessed letters, remaining lives.
///
/// Must contain `display` verbatim and each letter of `guessed` (the e2e
/// test relies on that). Layout is Agent C's choice; snapshot tests pin it.
pub fn board(lives_left: usize, display: &str, guessed: &[char]) -> String {
    let g: String = if guessed.is_empty() {
        String::from("-")
    } else {
        guessed
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    };
    format!(
        "{figure}\n\nWord: {display}\nGuessed: {g}\nLives: {lives_left}/{MAX_LIVES}",
        figure = hangman(lives_left),
    )
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

/// Print `text` (plus a newline) to stdout.
pub fn print(text: &str) {
    println!("{text}");
}

/// Print `prompt` to stdout (no newline), then read one line from stdin,
/// trimmed. `Err(UnexpectedEof)` on end of input.
pub fn read_line(prompt: &str) -> io::Result<String> {
    print!("{prompt}");
    io::stdout().flush()?;
    let line = std::io::stdin()
        .lines()
        .next()
        .transpose()?
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "end of input"))?;
    Ok(line.trim().to_string())
}

/// Ask a yes/no question, re-asking until the answer is `y`/`n`
/// (case-insensitive; `yes`/`no` accepted too).
pub fn confirm(prompt: &str) -> io::Result<bool> {
    loop {
        let answer = read_line(prompt)?;
        match answer.to_ascii_lowercase().as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => print("Please answer y or n."),
        }
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
    fn board_contains_display_and_guessed() {
        let b = board(3, "R _ S T _", &['a', 'd']);
        assert!(b.contains("R _ S T _"));
        assert!(b.contains('a'));
        assert!(b.contains('d'));
        assert!(b.contains("3/6"));
    }

    #[test]
    fn board_handles_empty_guesses() {
        assert!(board(6, "_ _ _", &[]).contains("_ _ _"));
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
