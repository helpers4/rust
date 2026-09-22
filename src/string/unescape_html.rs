// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::borrow::Cow;

/// Longest entity body we look for, e.g. `#x10FFFF` is 8 characters.
const MAX_ENTITY_LEN: usize = 10;

/// Decodes the HTML entities `&amp;`, `&lt;`, `&gt;`, `&quot;`, `&apos;`, `&#39;` and any numeric
/// character reference (`&#65;`, `&#x41;`).
///
/// It is the inverse of [`escape_html`](super::escape_html). The text is decoded in a single
/// pass, so `&amp;lt;` becomes `&lt;` and not `<`. Anything that is not a known entity, including
/// a numeric reference that is not a valid character, is left as it is. Returns the input
/// borrowed, without allocating, when it contains no `&`.
///
/// # Arguments
///
/// - `s` - The text to decode.
///
/// # Examples
///
/// ```
/// use helpers4::string::unescape_html;
///
/// assert_eq!(unescape_html("&lt;b&gt;Tom &amp; Jerry&lt;/b&gt;"), "<b>Tom & Jerry</b>");
/// assert_eq!(unescape_html("&#65;&#x42;"), "AB");
/// assert_eq!(unescape_html("&amp;lt;"), "&lt;");
/// assert_eq!(unescape_html("&unknown;"), "&unknown;");
/// ```
#[must_use]
pub fn unescape_html(s: &str) -> Cow<'_, str> {
    if !s.contains('&') {
        return Cow::Borrowed(s);
    }
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(start) = rest.find('&') {
        out.push_str(&rest[..start]);
        let tail = &rest[start..];
        if let Some((decoded, len)) = decode_entity(tail) {
            out.push(decoded);
            rest = &tail[len..];
        } else {
            out.push('&');
            rest = &tail[1..];
        }
    }
    out.push_str(rest);
    Cow::Owned(out)
}

/// Decodes the entity at the start of `tail` (which starts with `&`), returning the character
/// and the number of bytes the entity spans.
fn decode_entity(tail: &str) -> Option<(char, usize)> {
    let (end, _) = tail
        .char_indices()
        .take(MAX_ENTITY_LEN + 2)
        .find(|&(_, c)| c == ';')?;
    let body = &tail[1..end];
    let decoded = match body {
        "amp" => '&',
        "lt" => '<',
        "gt" => '>',
        "quot" => '"',
        "apos" => '\'',
        _ => decode_numeric(body.strip_prefix('#')?)?,
    };
    Some((decoded, end + 1))
}

/// Decodes the digits of a numeric reference: decimal, or hexadecimal after `x`/`X`.
fn decode_numeric(digits: &str) -> Option<char> {
    let (radix, digits) = match digits.strip_prefix(['x', 'X']) {
        Some(hex) => (16, hex),
        None => (10, digits),
    };
    if digits.is_empty() || !digits.chars().all(|c| c.is_digit(radix)) {
        return None;
    }
    u32::from_str_radix(digits, radix)
        .ok()
        .filter(|&code| code != 0)
        .and_then(char::from_u32)
}

#[cfg(test)]
#[path = "unescape_html.test.rs"]
mod tests;

#[cfg(test)]
#[path = "unescape_html.spec.rs"]
mod spec;
