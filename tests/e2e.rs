//! End-to-end test for the binary, through the `HANGMAN_WORD` env seam.
//! Owned by **Agent D** (e2e) / **Agent E** (the front-end it drives).
//! The binary runs the TUI in its headless transport (piped stdin),
//! which preserves the line-based input semantics these tests script.
//! Contract: `docs/contracts/tui.md`.

use std::io::Write;
use std::process::{Command, Stdio};

/// Spawn the binary with `HANGMAN_WORD=CAT`, feed `input` on stdin, and
/// return (stdout, exited-0).
fn play(input: &str) -> (String, bool) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_hangman4"))
        .env("HANGMAN_WORD", "CAT")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn the game binary");
    child
        .stdin
        .take()
        .expect("piped stdin")
        .write_all(input.as_bytes())
        .expect("write the scripted input");
    let output = child
        .wait_with_output()
        .expect("wait for the game to finish");
    (
        String::from_utf8_lossy(&output.stdout).into_owned(),
        output.status.success(),
    )
}

#[test]
fn win_round() {
    // Easy, guess b (wrong), a, c, t (correct — win), no replay.
    // Contract test 1 listed only `1\nb\na\nc\nn\n`, which can never win on
    // CAT (`t` is never guessed); see the change request in PLAN.md.
    let (out, success) = play("1\nb\na\nc\nt\nn\n");
    assert!(success, "exit code should be 0, output:\n{out}");
    for expected in [
        "'b' is not in the word.",
        "'a' is in the word!",
        "'c' is in the word!",
        "'t' is in the word!",
        "You win!",
        "The word was: C A T",
    ] {
        assert!(out.contains(expected), "missing {expected:?} in:\n{out}");
    }
}

#[test]
fn loss_round() {
    // Easy, six wrong guesses (z y x w v u), no replay.
    let (out, success) = play("1\nz\ny\nx\nw\nv\nu\nn\n");
    assert!(success, "exit code should be 0, output:\n{out}");
    assert!(out.contains("You lose."), "missing 'You lose.' in:\n{out}");
    assert!(
        out.contains("The word was: C A T"),
        "missing the answer line in:\n{out}"
    );
}
