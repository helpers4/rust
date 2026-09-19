// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Uppercases the first character of `s` and leaves the rest untouched.
///
/// Unicode-aware: a character whose uppercase form is several characters
/// (`ß` -> `SS`) is expanded accordingly.
///
/// # Examples
///
/// ```
/// use helpers4::string::capitalize;
///
/// assert_eq!(capitalize("hello world"), "Hello world");
/// assert_eq!(capitalize(""), "");
/// ```
///
/// # Since
///
/// next
pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn empty_string_stays_empty() {
        assert_eq!(capitalize(""), "");
    }

    #[test]
    fn uppercases_first_ascii_letter_only() {
        assert_eq!(capitalize("hello World"), "Hello World");
    }

    #[test]
    fn expands_multi_char_uppercase() {
        assert_eq!(capitalize("ßa"), "SSa");
    }

    #[test]
    fn leaves_non_letters_alone() {
        assert_eq!(capitalize("1abc"), "1abc");
    }

    proptest! {
        #[test]
        fn is_idempotent(s in ".*") {
            let once = capitalize(&s);
            prop_assert_eq!(capitalize(&once), once);
        }

        #[test]
        fn preserves_tail(s in ".+") {
            let first_len = s.chars().next().unwrap().len_utf8();
            let out = capitalize(&s);
            prop_assert!(out.ends_with(&s[first_len..]));
        }
    }
}
