// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{FontError, FontInfo, read_font_at};

/// Reads what a font file says about itself: its family (typographic first), style, PostScript
/// name, weight or `wght` range and whether it is italic. Reads TrueType and OpenType files
/// (`.ttf`, `.otf`), the first font of a collection (`.ttc`, see [`read_font_at`] for the
/// others) and WOFF files; it does not interpret glyphs and does not check checksums.
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
    read_font_at(bytes, 0)
}

#[cfg(test)]
#[path = "read_font.test.rs"]
mod tests;

#[cfg(test)]
#[path = "read_font.spec.rs"]
mod spec;
