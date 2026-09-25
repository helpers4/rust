// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// The Levenshtein edit distance between `a` and `b`: the fewest single-character insertions,
/// deletions or substitutions needed to turn one into the other.
///
/// Operates on `char`s (Unicode scalar values), not bytes or grapheme clusters.
///
/// # Arguments
///
/// - `a` - The first string.
/// - `b` - The second string.
/// - `case_sensitive` - Whether the comparison is case-sensitive.
///
/// # Returns
///
/// The edit distance; `0` when `a` and `b` are equal (after case-folding when `case_sensitive`
/// is `false`).
///
/// # Examples
///
/// ```
/// use helpers4::string::levenshtein_distance;
///
/// assert_eq!(levenshtein_distance("kitten", "sitting", true), 3);
/// assert_eq!(levenshtein_distance("Kitten", "kitten", false), 0);
/// ```
#[must_use]
pub fn levenshtein_distance(a: &str, b: &str, case_sensitive: bool) -> usize {
    let (a, b) = if case_sensitive {
        (a.to_string(), b.to_string())
    } else {
        (a.to_lowercase(), b.to_lowercase())
    };
    if a == b {
        return 0;
    }
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    if a.is_empty() {
        return b.len();
    }
    if b.is_empty() {
        return a.len();
    }

    let mut previous_row: Vec<usize> = (0..=b.len()).collect();
    for (i, &ca) in a.iter().enumerate() {
        let mut current_row = vec![i + 1];
        for (j, &cb) in b.iter().enumerate() {
            let deletion = previous_row[j + 1] + 1;
            let insertion = current_row[j] + 1;
            let substitution = previous_row[j] + usize::from(ca != cb);
            current_row.push(deletion.min(insertion).min(substitution));
        }
        previous_row = current_row;
    }
    previous_row[b.len()]
}

#[cfg(test)]
#[path = "levenshtein_distance.test.rs"]
mod tests;

#[cfg(test)]
#[path = "levenshtein_distance.spec.rs"]
mod spec;
