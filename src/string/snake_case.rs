// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_words::words;

/// Converts `s` to `snake_case`.
///
/// Splits words the same way as [`camel_case`](super::camel_case).
///
/// # Examples
///
/// ```
/// use helpers4::string::snake_case;
///
/// assert_eq!(snake_case("helloWorld"), "hello_world");
/// assert_eq!(snake_case("Hello World"), "hello_world");
/// assert_eq!(snake_case(""), "");
/// ```
///
/// # Since
///
/// next
pub fn snake_case(s: &str) -> String {
    words(s).join("_")
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn converts_common_formats() {
        assert_eq!(snake_case("helloWorld"), "hello_world");
        assert_eq!(snake_case("kebab-case-input"), "kebab_case_input");
        assert_eq!(snake_case("HTMLParser"), "html_parser");
    }

    #[test]
    fn empty_input_is_empty() {
        assert_eq!(snake_case(""), "");
    }

    proptest! {
        #[test]
        fn is_idempotent_on_ascii(s in "[ -~]*") {
            let once = snake_case(&s);
            prop_assert_eq!(snake_case(&once), once);
        }

        #[test]
        fn has_no_edge_or_doubled_separators(s in "[ -~]*") {
            let out = snake_case(&s);
            prop_assert!(!out.starts_with('_') && !out.ends_with('_') && !out.contains("__"));
        }
    }
}
