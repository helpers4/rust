// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::PercentDecodeError;
use std::borrow::Cow;

/// Decodes the `%XX` escapes of `input`.
///
/// Every `%` must be followed by two hexadecimal digits (either case). A `+` stays a `+`: turning
/// it into a space is a rule of HTML form encoding, applied by [`parse_query`](super::parse_query)
/// and not by URL components in general. Returns the input borrowed when it has no `%`.
///
/// # Arguments
///
/// - `input` - The text to decode.
///
/// # Errors
///
/// [`PercentDecodeError::InvalidEscape`] for a `%` not followed by two hex digits, and
/// [`PercentDecodeError::InvalidUtf8`] when the decoded bytes are not valid UTF-8.
///
/// # Examples
///
/// ```
/// use helpers4::url::percent_decode;
///
/// assert_eq!(percent_decode("caf%C3%A9 %26 more")?, "café & more");
/// assert!(percent_decode("100%").is_err());
/// # Ok::<(), helpers4::url::PercentDecodeError>(())
/// ```
pub fn percent_decode(input: &str) -> Result<Cow<'_, str>, PercentDecodeError> {
    if !input.contains('%') {
        return Ok(Cow::Borrowed(input));
    }
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut rest = bytes.iter().enumerate();
    while let Some((index, &byte)) = rest.next() {
        if byte == b'%' {
            let high = rest.next().and_then(|(_, b)| hex_value(*b));
            let low = rest.next().and_then(|(_, b)| hex_value(*b));
            match (high, low) {
                (Some(high), Some(low)) => out.push(high << 4 | low),
                _ => return Err(PercentDecodeError::InvalidEscape { index }),
            }
        } else {
            out.push(byte);
        }
    }
    String::from_utf8(out)
        .map(Cow::Owned)
        .map_err(|_| PercentDecodeError::InvalidUtf8)
}

fn hex_value(byte: u8) -> Option<u8> {
    char::from(byte)
        .to_digit(16)
        .and_then(|digit| u8::try_from(digit).ok())
}

#[cfg(test)]
#[path = "percent_decode.test.rs"]
mod tests;

#[cfg(test)]
#[path = "percent_decode.spec.rs"]
mod spec;
