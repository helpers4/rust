// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{_sfnt, _woff, FontError};
use std::borrow::Cow;

/// The font file as the sfnt that shapers and rasterizers read (`skrifa`, `ttf-parser`, ...):
/// a WOFF file is rebuilt with its tables decompressed; a TrueType, OpenType or collection file
/// is already one and comes back as it is, without a copy. A collection keeps all its fonts:
/// open it with the same index as [`read_font_at`](super::read_font_at).
///
/// Checksums are neither verified nor recomputed: the table checksums are the ones the WOFF
/// file records.
///
/// # Arguments
///
/// - `bytes` - The font file.
///
/// # Returns
///
/// The sfnt bytes, borrowed from `bytes` unless the file was WOFF.
///
/// # Errors
///
/// A [`FontError`] for data that is not a supported font, is truncated, or holds a WOFF table
/// that does not decompress to its announced size.
///
/// # Examples
///
/// ```
/// use helpers4::font::{to_sfnt, FontError};
///
/// let ttf = b"\0\x01\0\0\0\0\0\0\0\0\0\0";
/// assert_eq!(to_sfnt(ttf).unwrap().as_ref(), ttf);
/// assert_eq!(to_sfnt(b"wOF2....").err(), Some(FontError::UnsupportedWoff2));
/// ```
pub fn to_sfnt(bytes: &[u8]) -> Result<Cow<'_, [u8]>, FontError> {
    _sfnt::count(bytes)?;
    if bytes.starts_with(b"wOFF") {
        _woff::to_sfnt(bytes).map(Cow::Owned)
    } else {
        Ok(Cow::Borrowed(bytes))
    }
}

#[cfg(test)]
#[path = "to_sfnt.test.rs"]
mod tests;

#[cfg(test)]
#[path = "to_sfnt.spec.rs"]
mod spec;
