// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn display_describes_each_variant() {
    assert_eq!(FontError::Truncated.to_string(), "font data is truncated");
    assert_eq!(
        FontError::UnknownFormat.to_string(),
        "not a TrueType, OpenType or WOFF font"
    );
    assert_eq!(
        FontError::UnsupportedWoff2.to_string(),
        "WOFF2 fonts are not supported"
    );
    assert_eq!(
        FontError::InvalidCompressedData.to_string(),
        "invalid compressed WOFF table"
    );
    assert_eq!(
        FontError::MissingNameTable.to_string(),
        "the font has no name table"
    );
    assert_eq!(
        FontError::MissingFamily.to_string(),
        "the font has no family name"
    );
}

#[test]
fn is_a_std_error() {
    let error: &dyn std::error::Error = &FontError::Truncated;
    assert!(error.source().is_none());
}
