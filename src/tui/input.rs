//! Input tokens: reading one raw event per transport and mapping it to
//! a per-screen action. Part of the TUI (contract
//! `docs/contracts/tui.md`); see `mod.rs` for the state machine.

use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use super::Ctx;
use crate::words::Difficulty;

/// One raw input event from the active transport. TTY: crossterm events
/// filtered down to a key press. Headless: one trimmed line of stdin.
pub fn next_token(ctx: &Ctx) -> io::Result<Token> {
    if ctx.tui {
        loop {
            if let Event::Key(KeyEvent {
                code,
                kind: KeyEventKind::Press,
                ..
            }) = event::read()?
            {
                return Ok(Token::Key(code));
            }
        }
    }
    let line = std::io::stdin()
        .lines()
        .next()
        .transpose()?
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "end of input"))?;
    Ok(Token::Line(line.trim().to_string()))
}

/// One raw input event.
pub enum Token {
    Key(KeyCode),
    Line(String),
}

pub enum MenuAction {
    Pick(Difficulty),
    Quit,
    Ignore,
}

pub enum GameAction {
    Guess(char),
    Quit,
    Ignore,
}

pub enum AgainAction {
    Again,
    Quit,
    Ignore,
}

/// Pure token -> action mapping for the menu.
pub fn menu_action(token: Token) -> MenuAction {
    match token {
        Token::Key(KeyCode::Char('1')) => MenuAction::Pick(Difficulty::Easy),
        Token::Key(KeyCode::Char('2')) => MenuAction::Pick(Difficulty::Medium),
        Token::Key(KeyCode::Char('3')) => MenuAction::Pick(Difficulty::Hard),
        Token::Key(KeyCode::Esc) | Token::Key(KeyCode::Char('q')) => MenuAction::Quit,
        Token::Line(line) => match line.to_ascii_lowercase().as_str() {
            "" | "1" => MenuAction::Pick(Difficulty::Easy),
            "2" => MenuAction::Pick(Difficulty::Medium),
            "3" => MenuAction::Pick(Difficulty::Hard),
            "q" | "quit" => MenuAction::Quit,
            _ => MenuAction::Ignore,
        },
        _ => MenuAction::Ignore,
    }
}

/// Pure token -> action mapping for the game. `q` is a letter guess
/// like any other a-z letter (a word may contain Q); only `Esc` quits
/// a round on the TTY transport. The headless transport has no
/// mid-round quit, matching the retired CLI.
pub fn game_action(token: Token) -> GameAction {
    match token {
        Token::Key(KeyCode::Esc) => GameAction::Quit,
        Token::Key(KeyCode::Char(c)) => letter_action(c),
        Token::Line(line) => {
            let low = line.to_ascii_lowercase();
            match low.chars().count() {
                1 => letter_action(low.chars().next().unwrap()),
                _ => GameAction::Ignore,
            }
        }
        _ => GameAction::Ignore,
    }
}

/// One a-z letter (either case) is a guess; anything else is ignored.
fn letter_action(c: char) -> GameAction {
    if c.is_ascii_alphabetic() {
        GameAction::Guess(c.to_ascii_lowercase())
    } else {
        GameAction::Ignore
    }
}

/// Pure token -> action mapping for the play-again prompt.
pub fn again_action(token: Token) -> AgainAction {
    match token {
        Token::Key(KeyCode::Char('y')) => AgainAction::Again,
        Token::Key(KeyCode::Char('n')) | Token::Key(KeyCode::Esc) => AgainAction::Quit,
        Token::Line(line) => match line.to_ascii_lowercase().as_str() {
            "y" | "yes" => AgainAction::Again,
            "n" | "no" | "q" => AgainAction::Quit,
            _ => AgainAction::Ignore,
        },
        _ => AgainAction::Ignore,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_tokens() {
        assert!(matches!(
            menu_action(Token::Line("1".into())),
            MenuAction::Pick(Difficulty::Easy)
        ));
        assert!(matches!(
            menu_action(Token::Line("".into())),
            MenuAction::Pick(Difficulty::Easy)
        ));
        assert!(matches!(
            menu_action(Token::Line("2".into())),
            MenuAction::Pick(Difficulty::Medium)
        ));
        assert!(matches!(
            menu_action(Token::Line("3".into())),
            MenuAction::Pick(Difficulty::Hard)
        ));
        assert!(matches!(
            menu_action(Token::Line("q".into())),
            MenuAction::Quit
        ));
        assert!(matches!(
            menu_action(Token::Line("9".into())),
            MenuAction::Ignore
        ));
        assert!(matches!(
            menu_action(Token::Key(KeyCode::Char('3'))),
            MenuAction::Pick(Difficulty::Hard)
        ));
        assert!(matches!(
            menu_action(Token::Key(KeyCode::Esc)),
            MenuAction::Quit
        ));
    }

    #[test]
    fn game_tokens() {
        assert!(matches!(
            game_action(Token::Line("b".into())),
            GameAction::Guess('b')
        ));
        assert!(matches!(
            game_action(Token::Line("B".into())),
            GameAction::Guess('b')
        ));
        assert!(matches!(
            game_action(Token::Line("ab".into())),
            GameAction::Ignore
        ));
        assert!(matches!(
            game_action(Token::Line("q".into())),
            GameAction::Guess('q')
        ));
        assert!(matches!(
            game_action(Token::Key(KeyCode::Char('q'))),
            GameAction::Guess('q')
        ));
        assert!(matches!(
            game_action(Token::Key(KeyCode::Char('x'))),
            GameAction::Guess('x')
        ));
        assert!(matches!(
            game_action(Token::Key(KeyCode::Esc)),
            GameAction::Quit
        ));
        assert!(matches!(
            game_action(Token::Key(KeyCode::Char('5'))),
            GameAction::Ignore
        ));
    }

    #[test]
    fn again_tokens() {
        assert!(matches!(
            again_action(Token::Line("y".into())),
            AgainAction::Again
        ));
        assert!(matches!(
            again_action(Token::Line("yes".into())),
            AgainAction::Again
        ));
        assert!(matches!(
            again_action(Token::Line("n".into())),
            AgainAction::Quit
        ));
        assert!(matches!(
            again_action(Token::Line("no".into())),
            AgainAction::Quit
        ));
        assert!(matches!(
            again_action(Token::Line("maybe".into())),
            AgainAction::Ignore
        ));
        assert!(matches!(
            again_action(Token::Key(KeyCode::Char('y'))),
            AgainAction::Again
        ));
    }
}
