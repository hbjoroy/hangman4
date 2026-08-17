//! Terminal user interface: the game's only front-end.
//!
//! Contract: `docs/contracts/tui.md`.
//!
//! One state machine, two transports, chosen once in `run`:
//! - **TTY** — stdin and stdout are terminals: crossterm raw mode,
//!   alternate screen, hidden cursor; colored frames drawn in place
//!   (`frames.rs`).
//! - **Headless** — anything else (piped stdin, e.g. the e2e test): one
//!   line of stdin per input event (`input.rs`), plain-text frames with
//!   the same fixed strings.
//!
//! No mutable bindings anywhere: the shell is a rebind-free loop, the
//! round is a recursion over immutable states, and the token readers are
//! stateless filter loops. The `HANGMAN_WORD` environment variable is a
//! deterministic test seam for the e2e test.

mod frames;
mod input;

use std::env;
use std::io::{self, IsTerminal};

use crossterm::cursor::{Hide, Show};
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};

use crate::game::{GameState, LetterOutcome};
use crate::words::Difficulty;

use frames::{Frame, GameView, render};
use input::{
    AgainAction, GameAction, MenuAction, again_action, game_action, menu_action, next_token,
};

/// Transport selection, decided once in `run`.
struct Ctx {
    tui: bool,
}

/// Run the game until the player quits. `Ok(())` on a clean exit.
///
/// The terminal teardown runs explicitly on every exit path once raw
/// mode is enabled (a guard type could not work: its drop signature
/// requires a non-immutable receiver, which the zero-mutable policy
/// forbids).
pub fn run() -> io::Result<()> {
    let tui = io::stdout().is_terminal() && io::stdin().is_terminal();
    if !tui {
        return shell(&Ctx { tui: false });
    }
    let raw = enable_raw_mode();
    let raw_ok = raw.is_ok();
    let prepared = match raw {
        Ok(()) => crossterm::execute!(io::stdout(), EnterAlternateScreen, Hide),
        Err(err) => Err(err),
    };
    let result = match prepared {
        Ok(()) => shell(&Ctx { tui: true }),
        Err(err) => Err(err),
    };
    if raw_ok {
        let _ = crossterm::execute!(io::stdout(), Show, LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
    result
}

/// The outer loop: menu -> round -> ending -> menu, until the player
/// quits. Stateless: every iteration binds fresh values.
fn shell(ctx: &Ctx) -> io::Result<()> {
    while let Some(difficulty) = pick_difficulty(ctx)? {
        let first = round_word(difficulty);
        let final_state = match play_round(ctx, first, None)? {
            Some(s) => s,
            None => break,
        };
        let ending = Frame::Ending {
            won: final_state.is_won(),
            answer: final_state.answer().to_string(),
        };
        render(ctx, &ending)?;
        if !play_again(ctx)? {
            break;
        }
    }
    render(ctx, &Frame::Goodbye)?;
    Ok(())
}

/// Difficulty menu. Re-renders the hint on an invalid input.
/// `None` means the player quit.
fn pick_difficulty(ctx: &Ctx) -> io::Result<Option<Difficulty>> {
    render(ctx, &Frame::Menu { hint: None })?;
    loop {
        match menu_action(next_token(ctx)?) {
            MenuAction::Pick(d) => return Ok(Some(d)),
            MenuAction::Quit => return Ok(None),
            MenuAction::Ignore => {
                render(
                    ctx,
                    &Frame::Menu {
                        hint: Some("Please type 1, 2 or 3."),
                    },
                )?;
            }
        }
    }
}

/// One round: render the board (with the previous feedback carried
/// over), read a guess, recurse until the game is over or the player
/// quits. One frame per state, so the recursion tracks the number of
/// accepted guesses. `None` means the player quit.
fn play_round(
    ctx: &Ctx,
    state: GameState,
    last: Option<(LetterOutcome, char)>,
) -> io::Result<Option<GameState>> {
    let view = GameView {
        state: state.clone(),
        last,
        hint: None,
    };
    render(ctx, &Frame::Game(view))?;
    let letter = loop {
        match game_action(next_token(ctx)?) {
            GameAction::Guess(c) => break c,
            GameAction::Quit => return Ok(None),
            GameAction::Ignore => {
                let view = GameView {
                    state: state.clone(),
                    last,
                    hint: Some("Please type a single letter a-z."),
                };
                render(ctx, &Frame::Game(view))?;
            }
        }
    };
    let (outcome, next) = state.guess(letter);
    if next.is_over() {
        let view = GameView {
            state: next.clone(),
            last: Some((outcome, letter)),
            hint: None,
        };
        render(ctx, &Frame::Game(view))?;
        Ok(Some(next))
    } else {
        play_round(ctx, next, Some((outcome, letter)))
    }
}

/// Ask whether to play again: `true` = another round, `false` = quit.
fn play_again(ctx: &Ctx) -> io::Result<bool> {
    loop {
        match again_action(next_token(ctx)?) {
            AgainAction::Again => return Ok(true),
            AgainAction::Quit => return Ok(false),
            AgainAction::Ignore => {}
        }
    }
}

/// Word for the next round. If the `HANGMAN_WORD` env var is set and
/// valid (1..=26 a–z letters, either case) it wins — the deterministic
/// e2e seam; otherwise a random word from the difficulty pool.
fn round_word(difficulty: Difficulty) -> GameState {
    match env::var("HANGMAN_WORD") {
        Ok(word) if is_valid_word(&word) => GameState::new(&word),
        _ => GameState::random(difficulty),
    }
}

fn is_valid_word(word: &str) -> bool {
    !word.is_empty() && word.chars().count() <= 26 && word.chars().all(|c| c.is_ascii_alphabetic())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn word_seam_validator() {
        assert!(is_valid_word("cat"));
        assert!(is_valid_word("Cat"));
        assert!(!is_valid_word(""));
        assert!(!is_valid_word("a b"));
        assert!(!is_valid_word("c4t"));
        assert!(!is_valid_word(&"a".repeat(27)));
        assert!(is_valid_word(&"a".repeat(26)));
    }
}
