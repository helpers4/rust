// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_words::words;

/// Converts `s` to `kebab-case`.
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
/// use helpers4::string::kebab_case;
///
/// assert_eq!(kebab_case("helloWorld"), "hello-world");
/// assert_eq!(kebab_case("user_name"), "user-name");
/// assert_eq!(kebab_case(""), "");
/// ```
#[must_use]
pub fn kebab_case(s: &str) -> String {
    words(s).join("-")
}

#[cfg(test)]
#[path = "kebab_case.test.rs"]
mod tests;

#[cfg(test)]
#[path = "kebab_case.spec.rs"]
mod spec;
