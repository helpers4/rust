// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_words::words;
use super::capitalize::capitalize;

/// Converts `s` to `PascalCase`.
///
/// Splits words the same way as [`camel_case`](super::camel_case).
///
/// # Examples
///
/// ```
/// use helpers4::string::pascal_case;
///
/// assert_eq!(pascal_case("hello-world"), "HelloWorld");
/// assert_eq!(pascal_case("user_name"), "UserName");
/// assert_eq!(pascal_case(""), "");
/// ```
///
/// # Since
///
/// next
pub fn pascal_case(s: &str) -> String {
    words(s).iter().map(|word| capitalize(word)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn converts_common_formats() {
        assert_eq!(pascal_case("hello-world"), "HelloWorld");
        assert_eq!(pascal_case("user_name"), "UserName");
        assert_eq!(pascal_case("camelCase"), "CamelCase");
        assert_eq!(pascal_case("userID"), "UserId");
    }

    #[test]
    fn empty_input_is_empty() {
        assert_eq!(pascal_case(""), "");
    }

    proptest! {
        // Not idempotent by design (see camel_case): only output shape is checked.
        #[test]
        fn has_no_separators(s in "[ -~]*") {
            prop_assert!(pascal_case(&s).chars().all(|c| c.is_ascii_alphanumeric()));
        }
    }
}
