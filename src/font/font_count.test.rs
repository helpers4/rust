// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::super::_test_support::*;
use super::*;

#[test]
fn counts_one_font_in_a_single_font_file() {
    let tables = typical_tables("Inter", "Bold", 700, 0);
    assert_eq!(font_count(&sfnt(&tables)), Ok(1));
    assert_eq!(font_count(&woff(&tables)), Ok(1));
}

#[test]
fn counts_the_fonts_of_a_collection() {
    let tables = typical_tables("Inter", "Bold", 700, 0);
    assert_eq!(font_count(&collection_of(&[&tables, &tables])), Ok(2));
    assert_eq!(font_count(&collection_of(&[])), Ok(0));
}

#[test]
fn refuses_what_is_not_a_supported_font() {
    assert_eq!(font_count(b"abc"), Err(FontError::Truncated));
    assert_eq!(font_count(b"ttcf\0\x01\0\0"), Err(FontError::Truncated));
    assert_eq!(
        font_count(b"wOF2abcdefgh"),
        Err(FontError::UnsupportedWoff2)
    );
    assert_eq!(font_count(b"%PDF-1.4"), Err(FontError::UnknownFormat));
}
