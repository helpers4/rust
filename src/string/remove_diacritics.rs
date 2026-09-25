// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use unicode_normalization::UnicodeNormalization;
use unicode_normalization::char::is_combining_mark;

/// Removes diacritical marks (accents) from `s`, e.g. `"café"` → `"cafe"`.
///
/// Works by Unicode-decomposing each character into its base letter plus combining marks
/// (`'é'` → `'e'` followed by a combining acute accent), then dropping the marks. Needs the
/// `string-diacritics` feature (not `string` alone, and not in `default`): full Unicode
/// normalization needs a real decomposition table, unlike the rest of this module, which is
/// dependency-free.
///
/// # Arguments
///
/// - `s` - The text to strip diacritics from.
///
/// # Returns
///
/// `s` with diacritics removed; unchanged if it has none.
///
/// # Examples
///
/// ```
/// use helpers4::string::remove_diacritics;
///
/// assert_eq!(remove_diacritics("café"), "cafe");
/// assert_eq!(remove_diacritics("naïve"), "naive");
/// assert_eq!(remove_diacritics("ÉCOLE"), "ECOLE");
/// assert_eq!(remove_diacritics("hello"), "hello");
/// ```
#[must_use]
pub fn remove_diacritics(s: &str) -> String {
    s.nfkd().filter(|&c| !is_combining_mark(c)).collect()
}

#[cfg(test)]
#[path = "remove_diacritics.test.rs"]
mod tests;

#[cfg(test)]
#[path = "remove_diacritics.spec.rs"]
mod spec;
