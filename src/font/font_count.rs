// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{_sfnt, FontError};

/// How many fonts a font file holds: the number of fonts of a collection (`.ttc`), 1 for a
/// TrueType, OpenType or WOFF file. Only the first bytes are read; the fonts themselves are not
/// parsed.
///
/// # Arguments
///
/// - `bytes` - The font file.
///
/// # Returns
///
/// The number of fonts, to use as the bound of the `index` of [`read_font_at`](super::read_font_at).
///
/// # Errors
///
/// A [`FontError`] for data that is not a supported font, or is truncated.
///
/// # Examples
///
/// ```
/// use helpers4::font::{font_count, FontError};
///
/// assert_eq!(font_count(b"not a font"), Err(FontError::UnknownFormat));
/// assert_eq!(font_count(b"\0\x01\0\0\0\0\0\0"), Ok(1));
/// ```
pub fn font_count(bytes: &[u8]) -> Result<usize, FontError> {
    _sfnt::count(bytes)
}

#[cfg(test)]
#[path = "font_count.test.rs"]
mod tests;
