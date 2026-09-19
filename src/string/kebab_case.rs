// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_words::words;

/// Converts `s` to `kebab-case`.
///
/// Splits words the same way as [`camel_case`](super::camel_case).
///
/// # Examples
///
/// ```
/// use helpers4::string::kebab_case;
///
/// assert_eq!(kebab_case("helloWorld"), "hello-world");
/// assert_eq!(kebab_case("user_name"), "user-name");
/// assert_eq!(kebab_case(""), "");
/// ```
///
/// # Since
///
/// next
pub fn kebab_case(s: &str) -> String {
    words(s).join("-")
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn converts_common_formats() {
        assert_eq!(kebab_case("helloWorld"), "hello-world");
        assert_eq!(kebab_case("snake_case_input"), "snake-case-input");
        assert_eq!(kebab_case("Hello World"), "hello-world");
    }

    #[test]
    fn empty_input_is_empty() {
        assert_eq!(kebab_case(""), "");
    }

    proptest! {
        #[test]
        fn is_idempotent_on_ascii(s in "[ -~]*") {
            let once = kebab_case(&s);
            prop_assert_eq!(kebab_case(&once), once);
        }
    }
}
