# Retired — `src/cli.rs` (Agent D)

Retired by the TUI iteration (Agent E): the stdio REPL module is gone;
`src/tui/` is the game's only front-end. The behaviors this contract
pinned survive in the TUI:

- the `HANGMAN_WORD` env seam (now in `tui`),
- the line-based input semantics (`1`/`2`/`3`, one letter, `y`/`n`) —
  now the TUI's **headless transport**,
- the fixed message strings (now sourced from `ui` in both transports),
- the e2e tests in `tests/e2e.rs` (unchanged; they drive the headless
  transport).

Replacement contract: `docs/contracts/tui.md`.
