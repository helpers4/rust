// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::get;

/// Returns `key` in dotenv `content` parsed as a boolean, or `default` when it is absent or not
/// recognized.
///
/// Recognized, case-insensitively: `true`/`false`, `1`/`0`, `yes`/`no`, `on`/`off`.
///
/// # Arguments
///
/// - `content` - The dotenv text to read.
/// - `key` - The variable name to look up.
/// - `default` - Returned when `key` is absent or its value is not one of the recognized words.
///
/// # Returns
///
/// The parsed boolean, or `default`.
///
/// # Examples
///
/// ```
/// use helpers4::env::get_bool;
///
/// let content = "DEBUG=on\nSTRICT=0\n";
/// assert!(get_bool(content, "DEBUG", false));
/// assert!(!get_bool(content, "STRICT", true));
/// assert!(get_bool(content, "MISSING", true));
/// ```
#[must_use]
pub fn get_bool(content: &str, key: &str, default: bool) -> bool {
    match get(content, key)
        .as_deref()
        .map(str::to_ascii_lowercase)
        .as_deref()
    {
        Some("true" | "1" | "yes" | "on") => true,
        Some("false" | "0" | "no" | "off") => false,
        _ => default,
    }
}

#[cfg(test)]
#[path = "get_bool.test.rs"]
mod tests;

#[cfg(test)]
#[path = "get_bool.spec.rs"]
mod spec;
