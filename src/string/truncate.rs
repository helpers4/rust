// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Shortens `s` to at most `max_chars` characters, ending with `suffix` when it was cut.
///
/// The suffix counts toward the limit. Lengths are in Unicode scalar values (`char`s), not
/// grapheme clusters. If the suffix alone does not fit, the first `max_chars` characters of the
/// suffix are returned.
///
/// # Examples
///
/// ```
/// use helpers4::string::truncate;
///
/// assert_eq!(truncate("Hello, world", 8, "..."), "Hello...");
/// assert_eq!(truncate("short", 8, "..."), "short");
/// assert_eq!(truncate("Hello", 2, "..."), "..");
/// ```
///
/// # Since
///
/// next
pub fn truncate(s: &str, max_chars: usize, suffix: &str) -> String {
    if s.chars().count() <= max_chars {
        return s.to_string();
    }
    let suffix_len = suffix.chars().count();
    if suffix_len >= max_chars {
        return suffix.chars().take(max_chars).collect();
    }
    let mut out: String = s.chars().take(max_chars - suffix_len).collect();
    out.push_str(suffix);
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn returns_input_when_it_fits() {
        assert_eq!(truncate("abc", 3, "..."), "abc");
        assert_eq!(truncate("", 0, "..."), "");
    }

    #[test]
    fn cuts_and_appends_suffix_within_the_limit() {
        assert_eq!(truncate("Hello, world", 8, "..."), "Hello...");
        assert_eq!(truncate("abcdef", 4, ""), "abcd");
    }

    #[test]
    fn suffix_longer_than_limit_is_itself_truncated() {
        assert_eq!(truncate("Hello", 2, "..."), "..");
        assert_eq!(truncate("Hello", 3, "..."), "...");
        assert_eq!(truncate("Hello", 0, "..."), "");
    }

    #[test]
    fn counts_chars_not_bytes() {
        assert_eq!(truncate("éééé", 3, "…"), "éé…");
    }

    proptest! {
        #[test]
        fn never_exceeds_the_limit(s in ".*", max in 0usize..40, suffix in ".{0,5}") {
            prop_assert!(truncate(&s, max, &suffix).chars().count() <= max);
        }

        #[test]
        fn is_identity_when_input_fits(s in ".{0,20}", suffix in ".{0,5}") {
            let max = s.chars().count();
            prop_assert_eq!(truncate(&s, max, &suffix), s);
        }
    }
}
