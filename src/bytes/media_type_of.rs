// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

fn starts_with_at(bytes: &[u8], signature: &[u8], offset: usize) -> bool {
    bytes
        .get(offset..)
        .is_some_and(|rest| rest.starts_with(signature))
}

/// Skips what may come before the `<svg` element: a byte order mark, whitespace, an XML
/// declaration, comments and a doctype.
fn svg_body(text: &str) -> &str {
    let mut rest = text.strip_prefix('\u{feff}').unwrap_or(text).trim_start();
    loop {
        let skipped = [("<?", "?>"), ("<!--", "-->"), ("<!", ">")]
            .iter()
            .find_map(|(open, close)| {
                let tail = rest.strip_prefix(open)?;
                tail.find(close).map(|end| &tail[end + close.len()..])
            });
        match skipped {
            Some(tail) => rest = tail.trim_start(),
            None => return rest,
        }
    }
}

fn is_svg(bytes: &[u8]) -> bool {
    let head = &bytes[..bytes.len().min(2048)];
    // A truncated multi-byte character at the cut is dropped; the prolog is ASCII.
    let text = match std::str::from_utf8(head) {
        Ok(text) => text,
        Err(error) => std::str::from_utf8(&head[..error.valid_up_to()]).unwrap_or(""),
    };
    let body = svg_body(text);
    body.get(..4)
        .is_some_and(|tag| tag.eq_ignore_ascii_case("<svg"))
        && body[4..]
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_whitespace() || c == '>' || c == '/')
}

/// Finds the media type of an image from its first bytes (its magic number), whatever its name
/// says: PNG, JPEG, GIF, WebP, AVIF and SVG. SVG has no magic number, so it is recognized as
/// text whose first element is `<svg`, possibly after a byte order mark, whitespace, an XML
/// declaration, comments or a doctype.
///
/// # Arguments
///
/// - `bytes` - The start of the file: 512 bytes are enough for every type but a long SVG prolog.
///
/// # Returns
///
/// The media type (`"image/png"`, ...), or `None` when it is none of these.
///
/// # Examples
///
/// ```
/// use helpers4::bytes::media_type_of;
///
/// assert_eq!(media_type_of(&[0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]), Some("image/png"));
/// assert_eq!(media_type_of(b"<svg xmlns='http://www.w3.org/2000/svg'/>"), Some("image/svg+xml"));
/// assert_eq!(media_type_of(b"hello"), None);
/// ```
#[must_use]
pub fn media_type_of(bytes: &[u8]) -> Option<&'static str> {
    if bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a]) {
        Some("image/png")
    } else if bytes.starts_with(&[0xff, 0xd8, 0xff]) {
        Some("image/jpeg")
    } else if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        Some("image/gif")
    } else if bytes.starts_with(b"RIFF") && starts_with_at(bytes, b"WEBP", 8) {
        Some("image/webp")
    } else if starts_with_at(bytes, b"ftyp", 4)
        && (starts_with_at(bytes, b"avif", 8) || starts_with_at(bytes, b"avis", 8))
    {
        Some("image/avif")
    } else if is_svg(bytes) {
        Some("image/svg+xml")
    } else {
        None
    }
}

#[cfg(test)]
#[path = "media_type_of.test.rs"]
mod tests;

#[cfg(test)]
#[path = "media_type_of.spec.rs"]
mod spec;
