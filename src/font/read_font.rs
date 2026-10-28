// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::_sfnt::{self, be16};
use super::{FontError, FontInfo, is_italic, weight_of_style};

/// How well a `name` record fits: Windows English first, then any Unicode or Windows record, then
/// Macintosh. `None` for a platform whose strings are not decoded.
fn rank(platform: u16, language: u16) -> Option<u8> {
    match (platform, language) {
        (3, 0x0409) => Some(0),
        (0 | 3, _) => Some(1),
        (1, _) => Some(2),
        _ => None,
    }
}

/// The string of a `name` record: UTF-16BE on the Unicode and Windows platforms, Macintosh
/// Roman (only its ASCII part) on the Macintosh one.
fn decode_name(platform: u16, data: &[u8]) -> String {
    if platform == 1 {
        data.iter()
            .map(|&b| {
                if b.is_ascii() {
                    char::from(b)
                } else {
                    '\u{fffd}'
                }
            })
            .collect()
    } else {
        char::decode_utf16(
            data.chunks_exact(2)
                .map(|pair| u16::from_be_bytes([pair[0], pair[1]])),
        )
        .map(|c| c.unwrap_or('\u{fffd}'))
        .collect()
    }
}

/// The best string for a name id, `None` when the table has none.
fn name(table: &[u8], id: u16) -> Option<String> {
    let header = table.get(..6)?;
    let count = usize::from(be16(header, 2));
    let storage = usize::from(be16(header, 4));
    let mut best: Option<(u8, String)> = None;
    for index in 0..count {
        let Some(record) = table.get(6 + 12 * index..6 + 12 * (index + 1)) else {
            break;
        };
        let (platform, language) = (be16(record, 0), be16(record, 4));
        if be16(record, 6) != id {
            continue;
        }
        let Some(rank) = rank(platform, language) else {
            continue;
        };
        let start = storage + usize::from(be16(record, 10));
        let end = start + usize::from(be16(record, 8));
        let Some(data) = table.get(start..end) else {
            continue;
        };
        if best.as_ref().is_none_or(|(known, _)| rank < *known) {
            best = Some((rank, decode_name(platform, data)));
        }
    }
    best.map(|(_, text)| text)
}

/// The weight class and whether the style flags say italic or oblique, from the `OS/2` table.
fn os2(table: &[u8]) -> (Option<f64>, bool) {
    let class = table.get(4..6).map(|b| u16::from_be_bytes([b[0], b[1]]));
    let weight = class.filter(|w| (1..=1000).contains(w)).map(f64::from);
    let italic = table
        .get(62..64)
        .is_some_and(|b| u16::from_be_bytes([b[0], b[1]]) & 0x0201 != 0);
    (weight, italic)
}

/// Whether the `head` table's style flags say bold, and italic.
fn mac_style(table: &[u8]) -> (bool, bool) {
    table
        .get(44..46)
        .map_or((false, false), |b| (b[1] & 1 != 0, b[1] & 2 != 0))
}

/// The range of the `wght` axis of an `fvar` table.
fn weight_range(table: &[u8]) -> Option<(f64, f64)> {
    let header = table.get(..12)?;
    let axes = usize::from(be16(header, 4));
    let count = usize::from(be16(header, 8));
    let size = usize::from(be16(header, 10));
    let fixed = |axis: &[u8], at: usize| {
        f64::from(i32::from_be_bytes([
            axis[at],
            axis[at + 1],
            axis[at + 2],
            axis[at + 3],
        ])) / 65536.0
    };
    (0..count).find_map(|index| {
        let start = axes + size * index;
        let record = table.get(start..start + 20)?;
        (&record[..4] == b"wght").then(|| (fixed(record, 4), fixed(record, 12)))
    })
}

/// Reads what a font file says about itself: its family (typographic first), style, PostScript
/// name, weight or `wght` range and whether it is italic. Reads TrueType and OpenType files
/// (`.ttf`, `.otf`), the first font of a collection (`.ttc`) and WOFF files; it does not
/// interpret glyphs and does not check checksums.
///
/// WOFF2 tables are Brotli-compressed, and this crate has no Brotli decoder: a WOFF2 file is a
/// [`FontError::UnsupportedWoff2`].
///
/// # Arguments
///
/// - `bytes` - The font file.
///
/// # Returns
///
/// The [`FontInfo`].
///
/// # Errors
///
/// A [`FontError`] for data that is not a supported font, is truncated, or has no `name` table
/// or no family name.
///
/// # Examples
///
/// ```
/// use helpers4::font::{read_font, FontError};
///
/// assert_eq!(read_font(b"not a font"), Err(FontError::UnknownFormat));
/// assert_eq!(read_font(b"wOF2...."), Err(FontError::UnsupportedWoff2));
/// ```
pub fn read_font(bytes: &[u8]) -> Result<FontInfo, FontError> {
    let tables = _sfnt::load(bytes)?;
    let names = tables.get(*b"name").ok_or(FontError::MissingNameTable)?;
    let family = name(names, 16)
        .or_else(|| name(names, 1))
        .ok_or(FontError::MissingFamily)?;
    let style = name(names, 17)
        .or_else(|| name(names, 2))
        .unwrap_or_default();

    let (os2_weight, os2_italic) = tables.get(*b"OS/2").map_or((None, false), os2);
    let (bold, head_italic) = tables.get(*b"head").map_or((false, false), mac_style);
    let weight = os2_weight.unwrap_or_else(|| if bold { 700.0 } else { weight_of_style(&style) });
    Ok(FontInfo {
        postscript_name: name(names, 6),
        weight,
        weight_range: tables.get(*b"fvar").and_then(weight_range),
        italic: os2_italic || head_italic || is_italic(&style),
        family,
        style,
    })
}

#[cfg(test)]
#[path = "read_font.test.rs"]
mod tests;

#[cfg(test)]
#[path = "read_font.spec.rs"]
mod spec;
