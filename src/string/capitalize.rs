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
#[path = "capitalize.test.rs"]
mod tests;

#[cfg(test)]
#[path = "capitalize.spec.rs"]
mod spec;
