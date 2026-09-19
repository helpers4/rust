// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_words::words;
use super::capitalize::capitalize;

/// Converts `s` to `camelCase`.
///
/// Words are split on any non-alphanumeric character and on case boundaries; an embedded run
/// of capitals is an acronym, so only its last letter starts the next word (`userID` becomes
/// `userId`).
///
/// # Examples
///
/// ```
/// use helpers4::string::camel_case;
///
/// assert_eq!(camel_case("hello-world"), "helloWorld");
/// assert_eq!(camel_case("user_name"), "userName");
/// assert_eq!(camel_case("userID"), "userId");
/// assert_eq!(camel_case(""), "");
/// ```
///
/// # Since
///
/// next
pub fn camel_case(s: &str) -> String {
    let mut iter = words(s).into_iter();
    let mut out = iter.next().unwrap_or_default();
    for word in iter {
        out.push_str(&capitalize(&word));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn converts_common_formats() {
        assert_eq!(camel_case("hello-world"), "helloWorld");
        assert_eq!(camel_case("user_name"), "userName");
        assert_eq!(camel_case("Hello World"), "helloWorld");
        assert_eq!(camel_case("PascalCase"), "pascalCase");
    }

    #[test]
    fn treats_embedded_acronyms_as_boundaries() {
        assert_eq!(camel_case("userID"), "userId");
    }

    #[test]
    fn empty_or_separator_only_input_is_empty() {
        assert_eq!(camel_case(""), "");
        assert_eq!(camel_case("--"), "");
    }

    #[test]
    fn single_word_is_lowercased() {
        assert_eq!(camel_case("HELLO"), "hello");
    }

    // Not idempotent by design: consecutive single-letter words (`A0A` -> `a0AA`) merge into an
    // acronym run on the next pass, so only output shape is checked here.
    proptest! {
        #[test]
        fn has_no_separators_and_starts_lowercase(s in "[ -~]*") {
            let out = camel_case(&s);
            prop_assert!(out.chars().all(|c| c.is_ascii_alphanumeric()));
            prop_assert!(!out.starts_with(|c: char| c.is_ascii_uppercase()));
        }
    }
}
