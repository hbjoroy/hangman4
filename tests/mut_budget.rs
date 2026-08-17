//! Enforces the project-wide policy of zero mutable bindings.
//!
//! Scans every `src/*.rs` file and fails if the source contains the
//! three-letter identifier token spelled `m` `u` `t` as a standalone word
//! (i.e. not as part of a longer identifier such as "mutable"). Runs as
//! part of `cargo test`, so every agent's local check enforces it.

use std::fs;
use std::path::PathBuf;

const BANNED: [char; 3] = ['m', 'u', 't'];

/// True if `text` contains the banned token as a standalone word.
fn contains_banned(text: &str) -> bool {
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();
    (0..n.saturating_sub(2)).any(|i| {
        chars[i..i + 3] == BANNED[..]
            && (i == 0 || !chars[i - 1].is_alphanumeric())
            && (i + 3 >= n || !chars[i + 3].is_alphanumeric())
    })
}

#[test]
fn src_files_contain_no_banned_token() {
    let src = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src");
    let files: Vec<PathBuf> = fs::read_dir(&src)
        .expect("read src/")
        .map(|e| e.expect("dir entry").path())
        .filter(|p| p.is_file() && p.extension().is_some_and(|e| e == "rs"))
        .collect();
    assert!(!files.is_empty(), "expected .rs files in src/");
    for path in &files {
        let text = fs::read_to_string(path).expect("read file");
        assert!(
            !contains_banned(&text),
            "forbidden token found in {}",
            path.display()
        );
    }
}
