// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::ParseDurationError;
use std::time::Duration;

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
    let bytes = input.as_bytes();
    let mut total = Duration::ZERO;
    let mut i = 0;
    let mut any = false;
    loop {
        i = run_end(bytes, i, u8::is_ascii_whitespace);
        if i == bytes.len() {
            break;
        }
        let digits_start = i;
        i = run_end(bytes, i, u8::is_ascii_digit);
        if i == digits_start {
            return Err(ParseDurationError::ExpectedNumber { index: i });
        }
        let amount: u64 = input[digits_start..i]
            .parse()
            .map_err(|_| ParseDurationError::Overflow)?;
        let unit_start = i;
        i = run_end(bytes, i, u8::is_ascii_alphabetic);
        if i == unit_start {
            return Err(ParseDurationError::MissingUnit { index: i });
        }
        let piece = match &input[unit_start..i] {
            "ms" => Some(Duration::from_millis(amount)),
            "s" => Some(Duration::from_secs(amount)),
            "m" => amount.checked_mul(60).map(Duration::from_secs),
            "h" => amount.checked_mul(3600).map(Duration::from_secs),
            "d" => amount.checked_mul(86_400).map(Duration::from_secs),
            "w" => amount.checked_mul(604_800).map(Duration::from_secs),
            _ => return Err(ParseDurationError::UnknownUnit { index: unit_start }),
        };
        total = piece
            .and_then(|piece| total.checked_add(piece))
            .ok_or(ParseDurationError::Overflow)?;
        any = true;
    }
    if any {
        Ok(total)
    } else {
        Err(ParseDurationError::Empty)
    }
}

/// Index just past the run of bytes, starting at `from`, that satisfy `keep`.
fn run_end(bytes: &[u8], from: usize, keep: fn(&u8) -> bool) -> usize {
    from + bytes[from..].iter().take_while(|byte| keep(byte)).count()
}

#[cfg(test)]
#[path = "parse.test.rs"]
mod tests;
