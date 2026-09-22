// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_words::words;
use super::capitalize::capitalize;

/// Converts `s` to `PascalCase`.
///
/// Splits words the same way as [`camel_case`](super::camel_case).
///
/// # Arguments
///
/// - `s` - The text to convert.
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
#[must_use]
pub fn pascal_case(s: &str) -> String {
    words(s).iter().map(|word| capitalize(word)).collect()
}

#[cfg(test)]
#[path = "pascal_case.test.rs"]
mod tests;

#[cfg(test)]
#[path = "pascal_case.spec.rs"]
mod spec;
