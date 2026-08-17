//! Word data and selection. Owned by **Agent A**.
//!
//! Contract: `docs/contracts/words.md`.
//!
//! Pure data plus one random helper. No I/O, no dependency on the other
//! modules, no new crates.

/// Difficulty presets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Difficulty {
    /// 3–5 letter words.
    Easy,
    /// 6–8 letter words.
    Medium,
    /// 9+ letter words.
    Hard,
}

impl Difficulty {
    /// Word pool for this difficulty.
    ///
    /// Invariants (tested by Agent A): non-empty, every word is 1..=26
    /// lowercase a–z letters, no duplicates within a pool.
    pub fn pool(self) -> &'static [&'static str] {
        todo!("Agent A — pool({self:?}) — see docs/contracts/words.md")
    }
}

/// Easy pool: 3–5 letter words, at least 30 words.
pub const EASY: &[&str] = &[
    // TODO(A): fill with at least 30 lowercase a–z words, 3–5 letters each.
];

/// Medium pool: 6–8 letter words, at least 30 words.
pub const MEDIUM: &[&str] = &[
    // TODO(A): fill with at least 30 lowercase a–z words, 6–8 letters each.
];

/// Hard pool: 9+ letter words, at least 20 words.
pub const HARD: &[&str] = &[
    // TODO(A): fill with at least 20 lowercase a–z words, 9+ letters each.
];

/// Uniform random index in `0..len`. Panics if `len == 0`.
///
/// Implementation hint (no mutable state): `rand::thread_rng().gen_range(0..len)`
/// with `use rand::Rng;` in scope, since `gen_range` is a trait method.
pub fn random_index(len: usize) -> usize {
    todo!("Agent A — random_index({len}) — see docs/contracts/words.md")
}
