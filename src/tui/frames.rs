//! Frames: the layout of a screen and its drawing. Part of the TUI
//! (contract `docs/contracts/tui.md`); see `mod.rs` for the state
//! machine and `input.rs` for the token readers.

use std::io;

use crossterm::cursor::MoveTo;
use crossterm::style::{Attribute, Color, Print, SetAttribute, SetForegroundColor};

use super::Ctx;
use crate::game::{GameState, LetterOutcome, MAX_LIVES};
use crate::ui;

/// Frame height in rows (menu, game, ending, and goodbye all use it).
/// The frame-height unit test pins the layout invariant.
#[cfg_attr(not(test), allow(dead_code))]
const ROWS: usize = 14;
/// Padding width so in-place redraws erase the previous frame's content.
const WIDTH: usize = 44;
/// Top row of the frame on the TTY transport.
const TOP: u16 = 1;

/// What to draw, as plain data.
pub enum Frame {
    Menu { hint: Option<&'static str> },
    Game(GameView),
    Ending { won: bool, answer: String },
    Goodbye,
}

/// A mid-game screen: the board plus the last feedback and a hint.
pub struct GameView {
    pub state: GameState,
    pub last: Option<(LetterOutcome, char)>,
    pub hint: Option<&'static str>,
}

/// One row of a frame, with its TTY styling.
struct Line {
    text: String,
    color: Color,
    bold: bool,
}

fn styled(text: &str, color: Color, bold: bool) -> Line {
    Line {
        text: format!("{:<w$}", text, w = WIDTH),
        color,
        bold,
    }
}

/// The frame as exactly `ROWS` rows.
fn block(frame: &Frame) -> Vec<Line> {
    match frame {
        Frame::Menu { hint } => menu_block(*hint),
        Frame::Game(view) => game_block(view),
        Frame::Ending { won, answer } => ending_block(*won, answer),
        Frame::Goodbye => goodbye_block(),
    }
}

fn menu_block(hint: Option<&str>) -> Vec<Line> {
    vec![
        styled("HANGMAN 4", Color::Cyan, true),
        styled("", Color::White, false),
        styled("Pick a difficulty:", Color::White, true),
        styled("  1) Easy    (3-5 letters)", Color::White, false),
        styled("  2) Medium  (6-8 letters)", Color::White, false),
        styled("  3) Hard    (9+ letters)", Color::White, false),
        styled("", Color::White, false),
        styled("Pick [1]:", Color::Yellow, false),
        styled("q to quit", Color::DarkGrey, false),
        styled("", Color::White, false),
        hint_line(hint, Color::Yellow),
        styled("", Color::White, false),
        styled("", Color::White, false),
        styled("", Color::White, false),
    ]
}

fn game_block(view: &GameView) -> Vec<Line> {
    let lives = view.state.lives_left();
    let figure_color = if lives == MAX_LIVES {
        Color::DarkGrey
    } else {
        Color::Red
    };
    let figure: Vec<String> = ui::hangman(lives).lines().map(str::to_string).collect();
    let lives_row = format!("{:<12}Lives: {}", figure_row(&figure, 2), pips(lives));
    let feedback_row = match view.last {
        Some((outcome, letter)) => {
            let color = match outcome {
                LetterOutcome::Correct => Color::Green,
                LetterOutcome::Duplicate => Color::Yellow,
                LetterOutcome::Wrong => Color::Red,
            };
            styled(&ui::feedback(outcome, letter), color, false)
        }
        None => styled("", Color::White, false),
    };
    vec![
        styled("HANGMAN 4", Color::Cyan, true),
        styled("", Color::White, false),
        styled(&figure_row(&figure, 0), figure_color, false),
        styled(&figure_row(&figure, 1), figure_color, false),
        styled(&lives_row, figure_color, false),
        styled(&figure_row(&figure, 3), figure_color, false),
        styled(&figure_row(&figure, 4), figure_color, false),
        styled(&figure_row(&figure, 5), figure_color, false),
        styled("", Color::White, false),
        styled(
            &format!("Word:  {}", view.state.display()),
            Color::Green,
            true,
        ),
        styled(
            &format!("Guessed: {}", guessed_row(&view.state)),
            Color::Yellow,
            false,
        ),
        feedback_row,
        styled("", Color::White, false),
        styled(
            view.hint.unwrap_or("Guess a letter (a-z), esc to quit"),
            Color::White,
            false,
        ),
    ]
}

fn ending_block(won: bool, answer: &str) -> Vec<Line> {
    let lives = if won { MAX_LIVES } else { 0 };
    let banner_color = if won { Color::Green } else { Color::Red };
    let figure: Vec<String> = ui::hangman(lives).lines().map(str::to_string).collect();
    // The fixed two lines come from `ui::ending` (the e2e relies on them).
    let (banner, word_line) = match ui::ending(won, answer).split_once('\n') {
        Some((first, rest)) => (first.to_string(), rest.to_string()),
        None => (String::new(), String::new()),
    };
    vec![
        styled("HANGMAN 4", Color::Cyan, true),
        styled("", Color::White, false),
        styled(&figure_row(&figure, 0), Color::DarkGrey, false),
        styled(&figure_row(&figure, 1), Color::DarkGrey, false),
        styled(
            &format!("{:<12}Lives: {}", figure_row(&figure, 2), pips(lives)),
            Color::DarkGrey,
            false,
        ),
        styled(&figure_row(&figure, 3), Color::DarkGrey, false),
        styled(&figure_row(&figure, 4), Color::DarkGrey, false),
        styled(&figure_row(&figure, 5), Color::DarkGrey, false),
        styled("", Color::White, false),
        styled(&banner, banner_color, true),
        styled(&word_line, Color::White, true),
        styled("", Color::White, false),
        styled("", Color::White, false),
        styled("y to play again, n to quit", Color::Yellow, false),
    ]
}

fn goodbye_block() -> Vec<Line> {
    vec![
        styled("HANGMAN 4", Color::Cyan, true),
        styled("", Color::White, false),
        styled("", Color::White, false),
        styled("", Color::White, false),
        styled("", Color::White, false),
        styled("", Color::White, false),
        styled("", Color::White, false),
        styled("", Color::White, false),
        styled("", Color::White, false),
        styled("Goodbye!", Color::Cyan, true),
        styled("", Color::White, false),
        styled("", Color::White, false),
        styled("", Color::White, false),
        styled("", Color::White, false),
    ]
}

/// Draw the frame with the active transport. TTY: colored rows drawn in
/// place (padded to `WIDTH` so the redraw erases the previous frame).
/// Headless: plain text rows.
pub fn render(ctx: &Ctx, frame: &Frame) -> io::Result<()> {
    for (row, line) in block(frame).iter().enumerate() {
        if ctx.tui {
            crossterm::execute!(
                io::stdout(),
                MoveTo(0, TOP + row as u16),
                if line.bold {
                    SetAttribute(Attribute::Bold)
                } else {
                    SetAttribute(Attribute::Reset)
                },
                SetForegroundColor(line.color),
                Print(line.text.as_str()),
                SetForegroundColor(Color::Reset),
            )?;
        } else {
            println!("{}", line.text);
        }
    }
    Ok(())
}

fn figure_row(figure: &[String], i: usize) -> String {
    figure.get(i).cloned().unwrap_or_default()
}

/// Filled pips for the lives left, dots for the lives lost.
fn pips(lives: usize) -> String {
    format!("{}{}", "*".repeat(lives), ".".repeat(MAX_LIVES - lives))
}

/// Guessed letters joined with spaces, or `-` when none.
fn guessed_row(state: &GameState) -> String {
    if state.guessed_letters().is_empty() {
        String::from("-")
    } else {
        state
            .guessed_letters()
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn hint_line(hint: Option<&str>, color: Color) -> Line {
    match hint {
        Some(text) => styled(text, color, false),
        None => styled("", Color::White, false),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_block_shows_word_and_feedback() {
        let (outcome, state) = GameState::new("RUST").guess('s');
        let view = GameView {
            state,
            last: Some((outcome, 's')),
            hint: None,
        };
        let joined: String = block(&Frame::Game(view))
            .iter()
            .map(|l| l.text.clone())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(joined.contains("_ _ S _"));
        assert!(joined.contains("'s' is in the word!"));
        assert!(joined.contains("Guessed: s"));
    }

    #[test]
    fn ending_block_has_fixed_lines() {
        let rows = block(&Frame::Ending {
            won: true,
            answer: String::from("CAT"),
        });
        let joined: String = rows
            .iter()
            .map(|l| l.text.clone())
            .collect::<Vec<_>>()
            .join("\n");
        assert!(joined.contains("You win!"));
        assert!(joined.contains("The word was: C A T"));
    }

    #[test]
    fn every_frame_is_exactly_rows_rows() {
        let (outcome, state) = GameState::new("RUST").guess('s');
        let view = GameView {
            state,
            last: Some((outcome, 's')),
            hint: None,
        };
        let frames = [
            Frame::Menu { hint: None },
            Frame::Game(view),
            Frame::Ending {
                won: true,
                answer: String::from("CAT"),
            },
            Frame::Goodbye,
        ];
        for frame in frames.iter() {
            assert_eq!(block(frame).len(), ROWS, "wrong frame height");
        }
    }
}
