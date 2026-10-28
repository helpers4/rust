// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// Why a font file could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FontError {
    /// The data ends before a table directory, a table or a record it announces.
    Truncated,
    /// Not a TrueType, OpenType, TrueType collection or WOFF file.
    UnknownFormat,
    /// A WOFF2 file: its tables are Brotli-compressed, and this crate has no Brotli decoder.
    UnsupportedWoff2,
    /// A WOFF table whose compressed data is invalid or does not have its announced size.
    InvalidCompressedData,
    /// The file has no `name` table.
    MissingNameTable,
    /// The `name` table holds neither a typographic nor a plain family name.
    MissingFamily,
}

impl fmt::Display for FontError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated => write!(f, "font data is truncated"),
            Self::UnknownFormat => write!(f, "not a TrueType, OpenType or WOFF font"),
            Self::UnsupportedWoff2 => write!(f, "WOFF2 fonts are not supported"),
            Self::InvalidCompressedData => write!(f, "invalid compressed WOFF table"),
            Self::MissingNameTable => write!(f, "the font has no name table"),
            Self::MissingFamily => write!(f, "the font has no family name"),
        }
    }
}

impl std::error::Error for FontError {}

#[cfg(test)]
#[path = "font_error.test.rs"]
mod tests;
