// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_words::words;

/// Converts `s` to `CONSTANT_CASE` (also known as `SCREAMING_SNAKE_CASE`).
///
/// Splits words the same way as [`camel_case`](super::camel_case).
///
/// # Examples
///
/// ```
/// use helpers4::string::constant_case;
///
/// assert_eq!(constant_case("helloWorld"), "HELLO_WORLD");
/// assert_eq!(constant_case("max retries"), "MAX_RETRIES");
/// assert_eq!(constant_case(""), "");
/// ```
#[must_use]
pub fn constant_case(s: &str) -> String {
    words(s).join("_").to_uppercase()
}

#[cfg(test)]
#[path = "constant_case.test.rs"]
mod tests;

#[cfg(test)]
#[path = "constant_case.spec.rs"]
mod spec;
