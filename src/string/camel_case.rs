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
/// # Arguments
///
/// - `s` - The text to convert.
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
#[must_use]
pub fn camel_case(s: &str) -> String {
    let mut iter = words(s).into_iter();
    let mut out = iter.next().unwrap_or_default();
    for word in iter {
        out.push_str(&capitalize(&word));
    }
    out
}

#[cfg(test)]
#[path = "camel_case.test.rs"]
mod tests;

#[cfg(test)]
#[path = "camel_case.spec.rs"]
mod spec;
