//! Hangman 4 — entry point.
//!
//! The binary is five lines of glue: the front-end lives in `tui`, all
//! pure state in `game`, the presentation content in `ui`, all data in
//! `words`. See `PLAN.md` for architecture and agent assignment.

mod game;
mod tui;
mod ui;
mod words;

fn main() {
    if let Err(err) = tui::run() {
        eprintln!("io error: {err}");
        std::process::exit(1);
    }
}
