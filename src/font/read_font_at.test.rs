// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

#![allow(clippy::float_cmp)]

use super::super::_test_support::*;
use super::*;

#[test]
fn reads_each_font_of_a_collection() {
    let regular = typical_tables("Inter", "Regular", 400, 0);
    let bold = typical_tables("Inter", "Bold", 700, 0);
    let mono = typical_tables("Inter Mono", "Light", 300, 0);
    let bytes = collection_of(&[&regular, &bold, &mono]);
    let read = |index| read_font_at(&bytes, index).unwrap();
    assert_eq!((read(0).style.as_str(), read(0).weight), ("Regular", 400.0));
    assert_eq!((read(1).style.as_str(), read(1).weight), ("Bold", 700.0));
    assert_eq!(read(2).family, "Inter Mono");
}

#[test]
fn a_single_font_file_holds_only_the_font_zero() {
    let tables = typical_tables("Inter", "Bold", 700, 0);
    for bytes in [sfnt(&tables), woff(&tables)] {
        assert_eq!(read_font_at(&bytes, 0).unwrap().family, "Inter");
        assert_eq!(read_font_at(&bytes, 1), Err(FontError::IndexOutOfRange));
    }
}

#[test]
fn refuses_an_index_past_the_end_of_a_collection() {
    let tables = typical_tables("Inter", "Bold", 700, 0);
    let bytes = collection_of(&[&tables, &tables]);
    assert!(read_font_at(&bytes, 1).is_ok());
    assert_eq!(read_font_at(&bytes, 2), Err(FontError::IndexOutOfRange));
}

#[test]
fn reports_what_read_font_reports() {
    assert_eq!(read_font_at(b"", 0), Err(FontError::Truncated));
    assert_eq!(read_font_at(b"%PDF-1.4", 0), Err(FontError::UnknownFormat));
    assert_eq!(
        read_font_at(b"wOF2abcdefgh", 0),
        Err(FontError::UnsupportedWoff2)
    );
}
