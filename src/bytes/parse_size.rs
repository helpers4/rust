// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::ParseSizeError;

/// Parses a human-written size such as `"1.5 KiB"`, `"10MB"` or `"512"` into a number of bytes.
///
/// The number is a non-negative decimal (a fraction is allowed, at most 18 decimal digits are
/// used, and the result is rounded down), optionally followed by whitespace and a unit, matched
/// without regard to case:
///
/// - none or `B`: bytes;
/// - `KiB`, `MiB`, `GiB`, `TiB`, `PiB`, `EiB`, and the single letters `K`, `M`, `G`, `T`, `P`,
///   `E`: powers of 1024;
/// - `KB`, `MB`, `GB`, `TB`, `PB`, `EB`: powers of 1000.
///
/// [`format_size`](super::format_size) writes the binary form back.
///
/// # Arguments
///
/// - `input` - The size to parse.
///
/// # Errors
///
/// Returns a [`ParseSizeError`] for an empty string, something other than a number where one is
/// expected, an unknown unit, or a size that does not fit in a `u64`.
///
/// # Examples
///
/// ```
/// use helpers4::bytes::parse_size;
///
/// assert_eq!(parse_size("1.5 KiB"), Ok(1536));
/// assert_eq!(parse_size("10MB"), Ok(10_000_000));
/// assert_eq!(parse_size("512"), Ok(512));
/// assert!(parse_size("ten").is_err());
/// ```
pub fn parse_size(input: &str) -> Result<u64, ParseSizeError> {
    let bytes = input.as_bytes();
    let mut i = skip(bytes, 0, u8::is_ascii_whitespace);
    if i == bytes.len() {
        return Err(ParseSizeError::Empty);
    }
    let whole_end = skip(bytes, i, u8::is_ascii_digit);
    if whole_end == i {
        return Err(ParseSizeError::ExpectedNumber { index: i });
    }
    let whole: u128 = input[i..whole_end]
        .parse()
        .map_err(|_| ParseSizeError::Overflow)?;
    i = whole_end;
    let mut fraction = 0u128;
    let mut denominator = 1u128;
    if bytes.get(i) == Some(&b'.') {
        let digits_start = i + 1;
        let digits_end = skip(bytes, digits_start, u8::is_ascii_digit);
        if digits_end == digits_start {
            return Err(ParseSizeError::ExpectedNumber {
                index: digits_start,
            });
        }
        for digit in input[digits_start..digits_end].bytes().take(18) {
            fraction = fraction * 10 + u128::from(digit - b'0');
            denominator *= 10;
        }
        i = digits_end;
    }
    let unit_start = skip(bytes, i, u8::is_ascii_whitespace);
    let unit_end = bytes.len()
        - bytes
            .iter()
            .rev()
            .take_while(|b| b.is_ascii_whitespace())
            .count();
    let multiplier = multiplier(&input[unit_start..unit_end.max(unit_start)])
        .ok_or(ParseSizeError::UnknownUnit { index: unit_start })?;
    let total = whole
        .checked_mul(multiplier)
        .and_then(|n| n.checked_add(fraction * multiplier / denominator))
        .ok_or(ParseSizeError::Overflow)?;
    u64::try_from(total).map_err(|_| ParseSizeError::Overflow)
}

/// Index of the first byte at or after `from` that does not satisfy `keep`.
fn skip(bytes: &[u8], from: usize, keep: fn(&u8) -> bool) -> usize {
    from + bytes[from..].iter().take_while(|byte| keep(byte)).count()
}

fn multiplier(unit: &str) -> Option<u128> {
    Some(match unit.to_ascii_lowercase().as_str() {
        "" | "b" => 1,
        "k" | "kib" => 1 << 10,
        "m" | "mib" => 1 << 20,
        "g" | "gib" => 1 << 30,
        "t" | "tib" => 1 << 40,
        "p" | "pib" => 1 << 50,
        "e" | "eib" => 1 << 60,
        "kb" => 1_000,
        "mb" => 1_000_000,
        "gb" => 1_000_000_000,
        "tb" => 1_000_000_000_000,
        "pb" => 1_000_000_000_000_000,
        "eb" => 1_000_000_000_000_000_000,
        _ => return None,
    })
}

#[cfg(test)]
#[path = "parse_size.test.rs"]
mod tests;

#[cfg(test)]
#[path = "parse_size.spec.rs"]
mod spec;
