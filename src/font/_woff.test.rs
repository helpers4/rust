// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::super::_test_fixtures::*;
use super::super::_test_support::*;
use super::*;

/// A WOFF with one `name` table stored as given: `stored` bytes in the file, `size` announced.
fn woff_with(stored: &[u8], size: u32) -> Vec<u8> {
    let mut bytes = woff(&[(*b"name", stored.to_vec())]);
    bytes[44 + 12..44 + 16].copy_from_slice(&size.to_be_bytes());
    bytes
}

#[test]
fn keeps_a_table_stored_as_it_is() {
    let bytes = woff(&[(*b"name", vec![1, 2, 3]), (*b"glyf", vec![9; 4])]);
    let tables = load(&bytes).unwrap();
    assert_eq!(tables.get(*b"name"), Some(&[1u8, 2, 3][..]));
    assert_eq!(tables.get(*b"glyf"), None);
}

#[test]
fn inflates_a_compressed_table() {
    let bytes = woff_with(
        NAME_TABLE_COMPRESSED,
        u32::try_from(NAME_TABLE_ORIGINAL_SIZE).unwrap(),
    );
    let tables = load(&bytes).unwrap();
    assert_eq!(
        tables.get(*b"name").map(<[u8]>::len),
        Some(NAME_TABLE_ORIGINAL_SIZE)
    );
}

#[test]
fn rejects_a_table_that_does_not_inflate_to_its_size() {
    let wrong_size = woff_with(NAME_TABLE_COMPRESSED, 501);
    assert_eq!(
        load(&wrong_size).err(),
        Some(FontError::InvalidCompressedData)
    );
    let garbage = woff_with(&[0xff; 20], 100);
    assert_eq!(load(&garbage).err(), Some(FontError::InvalidCompressedData));
}

#[test]
fn rejects_a_table_larger_than_its_original_size() {
    let bytes = woff_with(&[1; 20], 10);
    assert_eq!(load(&bytes).err(), Some(FontError::InvalidCompressedData));
}

#[test]
fn rejects_an_absurd_original_size() {
    let bytes = woff_with(NAME_TABLE_COMPRESSED, u32::MAX);
    assert_eq!(load(&bytes).err(), Some(FontError::InvalidCompressedData));
}

#[test]
fn rejects_truncated_files() {
    let bytes = woff(&[(*b"name", vec![1; 12])]);
    for len in 0..bytes.len() {
        assert_eq!(
            load(&bytes[..len]).err(),
            Some(FontError::Truncated),
            "{len}"
        );
    }
}

#[test]
fn finds_a_wanted_table_after_other_tables() {
    let bytes = woff(&[
        (*b"glyf", vec![9; 4]),
        (*b"head", vec![5, 6]),
        (*b"name", vec![1, 2, 3]),
    ]);
    let tables = load(&bytes).unwrap();
    assert_eq!(tables.get(*b"head"), Some(&[5u8, 6][..]));
    assert_eq!(tables.get(*b"name"), Some(&[1u8, 2, 3][..]));
}

#[test]
fn inflates_a_table_of_a_megabyte() {
    let bytes = woff_with(ZEROS_COMPRESSED, u32::try_from(ZEROS_SIZE).unwrap());
    let tables = load(&bytes).unwrap();
    assert_eq!(tables.get(*b"name").map(<[u8]>::len), Some(ZEROS_SIZE));
}

#[test]
fn a_table_stored_larger_than_its_size_is_refused_even_if_it_inflates() {
    // The data is a valid zlib stream of 11 bytes, but the file says the table is 11 bytes long
    // and stores 22.
    let bytes = woff_with(STORED, 11);
    assert_eq!(load(&bytes).err(), Some(FontError::InvalidCompressedData));
}
