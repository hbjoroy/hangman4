//! Word data and selection. Owned by **Agent A**.
//!
//! Contract: `docs/contracts/words.md`.
//!
//! Pure data plus one random helper. No I/O, no dependency on the other
//! modules, no new crates.

use rand::Rng;

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
        match self {
            Difficulty::Easy => EASY,
            Difficulty::Medium => MEDIUM,
            Difficulty::Hard => HARD,
        }
    }
}

/// Easy pool: 3–5 letter words, at least 30 words.
pub const EASY: &[&str] = &[
    "cat", "dog", "sun", "jam", "sky", "owl", "hip", "map", "net", "pin", "arc", "bee", "cob",
    "dew", "elm", "fox", "gap", "hen", "ice", "jab", "key", "log", "mud", "nut", "orb", "pen",
    "rod", "sip", "tan", "urn", "home", "tree", "fish", "jump", "blue", "vine", "grape", "tiger",
    "piano", "crane",
];

/// Medium pool: 6–8 letter words, at least 30 words.
pub const MEDIUM: &[&str] = &[
    "desert", "falcon", "garlic", "island", "jungle", "kernel", "magnet", "napkin", "orange",
    "pistol", "quiver", "rocket", "saddle", "tunnel", "velvet", "waffle", "chicken", "giraffe",
    "library", "machine", "novelty", "picture", "silence", "tempest", "village", "walrus",
    "elephant", "festival", "gardener", "infinite", "mountain", "question", "sunshine", "umbrella",
];

/// Hard pool: 9+ letter words, at least 20 words.
pub const HARD: &[&str] = &[
    "adventure",
    "butterfly",
    "chemistry",
    "crocodile",
    "education",
    "fireplace",
    "geography",
    "hamburger",
    "hurricane",
    "jellyfish",
    "knowledge",
    "landscape",
    "milestone",
    "nightmare",
    "pineapple",
    "supernova",
    "telescope",
    "waterfall",
    "xylophone",
    "yesterday",
    "chandelier",
    "government",
    "impossible",
    "laboratory",
    "remarkable",
    "magnificent",
    "opportunity",
    "neighborhood",
];

/// Uniform random index in `0..len`. Panics if `len == 0`.
///
/// Implementation hint (no mutable state): `rand::rng().random_range(0..len)`
/// with `use rand::Rng;` in scope, since `random_range` is a trait method.
/// (`rand::thread_rng().gen_range(..)` was renamed to `rand::rng().random_range(..)`
/// in rand 0.9.5; see the change request in PLAN.md.)
pub fn random_index(len: usize) -> usize {
    // `thread_rng`/`gen_range` were renamed to `rng`/`random_range` in
    // rand 0.9.5 (deprecation warnings would break `-D warnings` CI).
    rand::rng().random_range(0..len)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Pool invariants: non-empty, lowercase a–z only, length in band,
    /// no duplicates.
    fn check_pool(name: &str, pool: &[&str], min_len: usize, max_len: usize) {
        assert!(!pool.is_empty(), "{name} is empty");
        for word in pool {
            assert!(
                word.chars().all(|c| c.is_ascii_lowercase()),
                "{name}: {word:?} is not lowercase a–z"
            );
            assert!(
                (min_len..=max_len).contains(&word.len()),
                "{name}: {word:?} is outside the {min_len}–{max_len} band"
            );
        }
        for word in pool {
            let duplicates = pool.iter().filter(|w| *w == word).count();
            assert_eq!(duplicates, 1, "{name}: duplicate {word:?}");
        }
    }

    #[test]
    fn pools_satisfy_invariants() {
        check_pool("EASY", EASY, 3, 5);
        check_pool("MEDIUM", MEDIUM, 6, 8);
        check_pool("HARD", HARD, 9, 26);
    }

    #[test]
    fn pools_have_enough_words() {
        assert!(EASY.len() >= 30, "EASY has {} words", EASY.len());
        assert!(MEDIUM.len() >= 30, "MEDIUM has {} words", MEDIUM.len());
        assert!(HARD.len() >= 20, "HARD has {} words", HARD.len());
    }

    #[test]
    fn difficulty_pool_points_at_the_constants() {
        assert_eq!(Difficulty::Easy.pool(), EASY);
        assert_eq!(Difficulty::Medium.pool(), MEDIUM);
        assert_eq!(Difficulty::Hard.pool(), HARD);
    }

    #[test]
    fn random_index_of_one_is_zero() {
        for _ in 0..100 {
            assert_eq!(random_index(1), 0);
        }
    }

    #[test]
    fn random_index_of_two_is_binary() {
        for _ in 0..100 {
            let value = random_index(2);
            assert!(value == 0 || value == 1, "got {value}");
        }
    }

    #[test]
    #[should_panic]
    fn random_index_of_zero_panics() {
        random_index(0);
    }
}
