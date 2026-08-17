//! The REPL glue: input, the game core, and rendering. Owned by **Agent D**.
//!
//! Contract: `docs/contracts/cli.md`.
//!
//! No mutable bindings anywhere: the round is a recursion over immutable
//! states (at most 26 frames deep), not a loop over a changing binding.
//! The `HANGMAN_WORD` environment variable is a deterministic test seam
//! for the e2e test.

use std::io;

/// Run the game until the player quits. `Ok(())` on a clean exit.
pub fn run() -> io::Result<()> {
    todo!("Agent D — see docs/contracts/cli.md")
}
