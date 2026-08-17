//! Hangman 4 — entry point.
//!
//! The binary is five lines of glue: all logic lives in `cli`, all pure
//! state in `game`, all rendering in `ui`, all data in `words`.
//! See `PLAN.md` for architecture and agent assignment.

mod cli;
mod game;
mod ui;
mod words;

fn main() {
    if let Err(err) = cli::run() {
        eprintln!("io error: {err}");
        std::process::exit(1);
    }
}
