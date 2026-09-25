// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::get;
use crate::internal::duration_grammar;
use std::time::Duration;

/// Returns `key` in dotenv `content` parsed as a human-written duration (`"1h30m"`, `"2d"`,
/// `"500ms"`), or `default` when it is absent or does not parse.
///
/// Sharing this format with the `duration` module's own `parse` does not need that module
/// enabled: `env` stands alone, like every module here.
///
/// # Arguments
///
/// - `content` - The dotenv text to read.
/// - `key` - The variable name to look up.
/// - `default` - Returned when `key` is absent or its value does not parse.
///
/// # Returns
///
/// The parsed duration, or `default`.
///
/// # Examples
///
/// ```
/// use helpers4::env::get_duration;
/// use std::time::Duration;
///
/// let content = "TIMEOUT=1h30m\nNAME=not-a-duration\n";
/// assert_eq!(get_duration(content, "TIMEOUT", Duration::ZERO), Duration::from_secs(5400));
/// assert_eq!(get_duration(content, "NAME", Duration::ZERO), Duration::ZERO);
/// assert_eq!(get_duration(content, "MISSING", Duration::from_secs(30)), Duration::from_secs(30));
/// ```
#[must_use]
pub fn get_duration(content: &str, key: &str, default: Duration) -> Duration {
    get(content, key)
        .and_then(|v| duration_grammar::parse(&v).ok())
        .unwrap_or(default)
}

#[cfg(test)]
#[path = "get_duration.test.rs"]
mod tests;

#[cfg(test)]
#[path = "get_duration.spec.rs"]
mod spec;
