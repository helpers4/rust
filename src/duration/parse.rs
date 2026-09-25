// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::ParseDurationError;
use crate::internal::duration_grammar;
use std::time::Duration;

impl From<duration_grammar::Error> for ParseDurationError {
    fn from(error: duration_grammar::Error) -> Self {
        match error {
            duration_grammar::Error::Empty => Self::Empty,
            duration_grammar::Error::ExpectedNumber { index } => Self::ExpectedNumber { index },
            duration_grammar::Error::MissingUnit { index } => Self::MissingUnit { index },
            duration_grammar::Error::UnknownUnit { index } => Self::UnknownUnit { index },
            duration_grammar::Error::Overflow => Self::Overflow,
        }
    }
}

/// Parses a human-written duration such as `"1h30m"`, `"2d"` or `"500ms"`.
///
/// The input is one or more `<integer><unit>` pairs, optionally separated by whitespace, and their
/// sum is returned. Units are `ms`, `s`, `m`, `h`, `d` (24 hours) and `w` (7 days), and may
/// repeat or come in any order. A bare number, a fraction, a sign or an unknown unit is an error
/// with the byte offset where it was found. It is the inverse of [`format`](super::format).
///
/// # Arguments
///
/// - `input` - The human-written duration, such as `"1h30m"`.
///
/// # Errors
///
/// Returns a [`ParseDurationError`] when the input is empty, has something other than a number
/// where one is expected, a number without a unit, an unknown unit, or a total that does not fit
/// in a [`Duration`].
///
/// # Examples
///
/// ```
/// use helpers4::duration::parse;
/// use std::time::Duration;
///
/// assert_eq!(parse("1h30m"), Ok(Duration::from_secs(5400)));
/// assert_eq!(parse("2d 12h"), Ok(Duration::from_secs(216_000)));
/// assert_eq!(parse("1500ms"), Ok(Duration::from_millis(1500)));
/// assert!(parse("90").is_err());
/// ```
pub fn parse(input: &str) -> Result<Duration, ParseDurationError> {
    duration_grammar::parse(input).map_err(ParseDurationError::from)
}

#[cfg(test)]
#[path = "parse.test.rs"]
mod tests;
