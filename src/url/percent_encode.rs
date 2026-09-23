// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::borrow::Cow;

/// Percent-encodes `input` for use as one URL component: a path segment, a query key or value.
///
/// Only the unreserved characters of RFC 3986 (`A-Z a-z 0-9 - . _ ~`) are kept; every other byte,
/// including `/`, `?`, `&`, `=`, `+` and every byte of a non-ASCII character, becomes `%XX` in
/// upper case. It is stricter than JavaScript's `encodeURIComponent` (which also leaves `! * ' ( )`)
/// so the result is safe in any part of a URL. Returns the input borrowed when there is nothing to
/// encode.
///
/// # Arguments
///
/// - `input` - The text to encode.
///
/// # Returns
///
/// The encoded text.
///
/// # Examples
///
/// ```
/// use helpers4::url::percent_encode;
///
/// assert_eq!(percent_encode("a b&c=d"), "a%20b%26c%3Dd");
/// assert_eq!(percent_encode("café"), "caf%C3%A9");
/// assert_eq!(percent_encode("safe-text_1.~"), "safe-text_1.~");
/// ```
#[must_use]
pub fn percent_encode(input: &str) -> Cow<'_, str> {
    if input.bytes().all(is_unreserved) {
        return Cow::Borrowed(input);
    }
    let mut out = String::with_capacity(input.len() + 8);
    for byte in input.bytes() {
        if is_unreserved(byte) {
            out.push(char::from(byte));
        } else {
            out.push('%');
            out.push(char::from(HEX[usize::from(byte >> 4)]));
            out.push(char::from(HEX[usize::from(byte & 0x0f)]));
        }
    }
    Cow::Owned(out)
}

const HEX: &[u8; 16] = b"0123456789ABCDEF";

fn is_unreserved(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~')
}

#[cfg(test)]
#[path = "percent_encode.test.rs"]
mod tests;

#[cfg(test)]
#[path = "percent_encode.spec.rs"]
mod spec;
