// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Whether `text` matches a simple glob pattern in full: `*` matches any sequence of characters
/// (including none), `?` exactly one character, everything else is literal.
///
/// It is not a shell glob: no `[...]` classes, `{a,b}` alternatives, escapes or path-aware `**`,
/// just the two wildcards. Characters are Unicode scalar values (`char`), so `?` matches a whole
/// `é` or emoji. Matching is case-sensitive; lowercase both sides to ignore case. Runs in
/// `O(pattern × text)` at worst, without recursion or a regex engine.
///
/// # Arguments
///
/// - `pattern` - The glob pattern.
/// - `text` - The string to test.
///
/// # Returns
///
/// `true` if `text` matches `pattern`.
///
/// # Examples
///
/// ```
/// use helpers4::string::match_glob;
///
/// assert!(match_glob("*.fig", "design.fig"));
/// assert!(match_glob("frame-?", "frame-é"));
/// assert!(!match_glob("frame-?", "frame-12"));
/// assert!(match_glob("screen *", &"Screen 2".to_lowercase())); // ignore case
/// ```
#[must_use]
pub fn match_glob(pattern: &str, text: &str) -> bool {
    let pattern: Vec<char> = pattern.chars().collect();
    let text: Vec<char> = text.chars().collect();
    let (mut p, mut t) = (0, 0);
    // The last `*` seen and the text position it currently stands for: on a mismatch, let it
    // swallow one more character and retry from there.
    let mut star: Option<(usize, usize)> = None;
    while t < text.len() {
        match pattern.get(p) {
            Some('*') => {
                star = Some((p, t));
                p += 1;
            }
            Some(&c) if c == '?' || c == text[t] => {
                p += 1;
                t += 1;
            }
            _ => match star {
                Some((star_p, star_t)) => {
                    star = Some((star_p, star_t + 1));
                    p = star_p + 1;
                    t = star_t + 1;
                }
                None => return false,
            },
        }
    }
    pattern[p..].iter().all(|&c| c == '*')
}

#[cfg(test)]
#[path = "match_glob.test.rs"]
mod tests;

#[cfg(test)]
#[path = "match_glob.spec.rs"]
mod spec;
