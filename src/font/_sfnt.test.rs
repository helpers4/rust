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
    let tables = load(&bytes, 0).unwrap();
    assert_eq!(tables.get(*b"name"), Some(&[4u8, 5][..]));
    assert_eq!(tables.get(*b"head"), Some(&[6u8][..]));
    assert_eq!(tables.get(*b"glyf"), None);
    assert_eq!(tables.get(*b"fvar"), None);
}

#[test]
fn finds_the_tables_of_the_first_font_of_a_collection() {
    let bytes = collection(&[(*b"name", vec![7, 7])]);
    assert_eq!(load(&bytes, 0).unwrap().get(*b"name"), Some(&[7u8, 7][..]));
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
    assert!(matches!(load(&bytes, 0), Err(FontError::Truncated)));
    let mut ignored = sfnt(&[(*b"glyf", vec![1; 8])]);
    ignored.truncate(ignored.len() - 2);
    assert!(load(&ignored, 0).is_ok());
}

#[test]
fn counts_the_fonts_of_a_file() {
    let tables = [(*b"name", vec![1])];
    assert_eq!(count(&sfnt(&tables)), Ok(1));
    assert_eq!(count(&woff(&tables)), Ok(1));
    assert_eq!(count(&collection_of(&[&tables, &tables, &tables])), Ok(3));
    assert_eq!(count(b"ab"), Err(FontError::Truncated));
    assert_eq!(count(b"ttcf\0\x01\0\0\0\0"), Err(FontError::Truncated));
    assert_eq!(count(b"wOF2...."), Err(FontError::UnsupportedWoff2));
    assert_eq!(count(b"%PDF"), Err(FontError::UnknownFormat));
}

#[test]
fn finds_the_tables_of_any_font_of_a_collection() {
    let bytes = collection_of(&[
        &[(*b"name", vec![1, 1])],
        &[(*b"name", vec![2, 2])],
        &[(*b"name", vec![3, 3])],
    ]);
    for (index, byte) in [1u8, 2, 3].into_iter().enumerate() {
        let tables = load(&bytes, index).unwrap();
        assert_eq!(tables.get(*b"name"), Some(&[byte, byte][..]));
    }
}

#[test]
fn refuses_an_index_past_the_last_font() {
    let tables = [(*b"name", vec![1])];
    let missing = Some(FontError::IndexOutOfRange);
    assert_eq!(load(&sfnt(&tables), 1).err(), missing);
    assert_eq!(load(&woff(&tables), 1).err(), missing);
    assert_eq!(load(&collection_of(&[&tables, &tables]), 2).err(), missing);
    assert_eq!(load(&sfnt(&tables), usize::MAX).err(), missing);
}

#[test]
fn rejects_a_collection_whose_offset_list_is_cut() {
    let bytes = b"ttcf\0\x01\0\0\0\0\0\x02\0\0\0\x14";
    assert_eq!(load(bytes, 0).err(), Some(FontError::Truncated));
    assert_eq!(load(bytes, 1).err(), Some(FontError::Truncated));
}
