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
pub fn snake_case(s: &str) -> String {
    words(s).join("_")
}

#[cfg(test)]
#[path = "snake_case.test.rs"]
mod tests;

#[cfg(test)]
#[path = "snake_case.spec.rs"]
mod spec;
