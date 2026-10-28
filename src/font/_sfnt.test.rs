// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::super::_test_support::*;
use super::*;

#[test]
fn reads_big_endian_numbers() {
    assert_eq!(be16(&[0, 9, 1], 1), 0x0901);
    assert_eq!(be32(&[9, 0, 1, 2, 3], 1), 0x0001_0203);
}

#[test]
fn keeps_only_the_wanted_tables() {
    let bytes = sfnt(&[
        (*b"glyf", vec![1, 2, 3]),
        (*b"name", vec![4, 5]),
        (*b"head", vec![6]),
    ]);
    let tables = load(&bytes).unwrap();
    assert_eq!(tables.get(*b"name"), Some(&[4u8, 5][..]));
    assert_eq!(tables.get(*b"head"), Some(&[6u8][..]));
    assert_eq!(tables.get(*b"glyf"), None);
    assert_eq!(tables.get(*b"fvar"), None);
}

#[test]
fn finds_the_tables_of_the_first_font_of_a_collection() {
    let bytes = collection(&[(*b"name", vec![7, 7])]);
    assert_eq!(load(&bytes).unwrap().get(*b"name"), Some(&[7u8, 7][..]));
}

#[test]
fn recognizes_the_wanted_tags() {
    for tag in [b"name", b"head", b"fvar"] {
        assert!(is_wanted(*tag));
    }
    assert!(is_wanted(*b"OS/2"));
    assert!(!is_wanted(*b"glyf"));
}

#[test]
fn rejects_a_table_that_ends_past_the_file() {
    let mut bytes = sfnt(&[(*b"name", vec![1; 8])]);
    bytes.truncate(bytes.len() - 2);
    assert!(matches!(load(&bytes), Err(FontError::Truncated)));
    let mut ignored = sfnt(&[(*b"glyf", vec![1; 8])]);
    ignored.truncate(ignored.len() - 2);
    assert!(load(&ignored).is_ok());
}
