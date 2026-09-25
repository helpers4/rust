// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::levenshtein_distance::levenshtein_distance;

/// Normalized [`levenshtein_distance`] between `a` and `b`, in `[0, 1]`.
///
/// `1` means identical, `0` means completely dissimilar relative to the longer string's length.
/// A convenience wrapper for scoring or ranking use cases.
///
/// # Arguments
///
/// - `a` - The first string.
/// - `b` - The second string.
/// - `case_sensitive` - Whether the comparison is case-sensitive.
///
/// # Returns
///
/// A similarity score between `0` and `1`; `1` when both strings are empty.
///
/// # Examples
///
/// ```
/// use helpers4::string::levenshtein_similarity;
///
/// assert_eq!(levenshtein_similarity("same", "same", true), 1.0);
/// assert!((levenshtein_similarity("kitten", "sitting", true) - 0.571_428_6).abs() < 1e-6);
/// ```
#[must_use]
pub fn levenshtein_similarity(a: &str, b: &str, case_sensitive: bool) -> f64 {
    // Case-fold here, once, and compare the folded strings from then on. Folding a single
    // character can *expand* it (Turkish 'İ' lowercases to two chars, 'i' + a combining dot):
    // computing max_len from the original `a`/`b` while `levenshtein_distance` compared the
    // folded ones could let distance / max_len exceed 1.
    let (a, b) = if case_sensitive {
        (a.to_string(), b.to_string())
    } else {
        (a.to_lowercase(), b.to_lowercase())
    };
    let max_len = a.chars().count().max(b.chars().count());
    if max_len == 0 {
        return 1.0;
    }
    #[allow(clippy::cast_precision_loss)]
    let ratio = levenshtein_distance(&a, &b, true) as f64 / max_len as f64;
    1.0 - ratio
}

#[cfg(test)]
#[path = "levenshtein_similarity.test.rs"]
mod tests;

#[cfg(test)]
#[path = "levenshtein_similarity.spec.rs"]
mod spec;
