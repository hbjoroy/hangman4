# Contract — `src/words.rs` (Agent A)

**Owns:** `src/words.rs`.
**May read besides its own file:** nothing.

## Responsibility
Word data and random selection. No I/O, no dependency on `game`/`ui`/`cli`,
no new crates (`rand` is already in `Cargo.toml`).

## Frozen public API

```rust
pub enum Difficulty { Easy, Medium, Hard }

impl Difficulty {
    /// Word pool for this difficulty.
    pub fn pool(self) -> &'static [&'static str];
}

/// 3–5 letter words. At least 30 words.
pub const EASY: &[&str];
/// 6–8 letter words. At least 30 words.
pub const MEDIUM: &[&str];
/// 9+ letter words. At least 20 words.
pub const HARD: &[&str];

/// Uniform random index in `0..len`. Panics if `len == 0`.
pub fn random_index(len: usize) -> usize;
```

Signatures are frozen. If one must change, log a change request under
"Change requests" in `PLAN.md` and work around it for now.

## Invariants
- Every pool is non-empty.
- Every word: lowercase a–z only, 1..=26 letters, within its pool's
  length band, no duplicates within a pool.
- `random_index` is uniform over its range.

## Implementation notes (no mutable bindings)
- `pool(self)`: `match self { Difficulty::Easy => EASY, … }`.
- `random_index`:

  ```rust
  use rand::Rng; // gen_range is a trait method
  rand::thread_rng().gen_range(0..len)
  ```

  `thread_rng()` returns an owned temporary; `gen_range(&mut self)`
  auto-refs it — no binding to hold, no mutable binding needed.

## Required tests (`#[cfg(test)] mod tests` in this file)
1. Pool invariants: for each of `EASY`/`MEDIUM`/`HARD` — non-empty; every
   word 1..=26 lowercase a–z letters; length in band (3–5 / 6–8 / 9+);
   no duplicates.
2. `random_index(1) == 0` over 100 samples.
3. `random_index(2)` yields only 0 or 1 over 100 samples.
4. `random_index(0)` panics (`#[should_panic]`).

## Done when
- `cargo test words` green
- `cargo clippy --all-targets -- -D warnings` clean
- `cargo fmt --check` clean
- `mut_budget` test green
