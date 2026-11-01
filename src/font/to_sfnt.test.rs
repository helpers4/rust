// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::super::_sfnt::be32;
use super::super::_test_fixtures::*;
use super::super::_test_support::*;
use super::super::read_font;
use super::*;

#[test]
fn an_sfnt_or_a_collection_comes_back_as_it_is() {
    let tables = typical_tables("Inter", "Bold", 700, 0);
    for bytes in [sfnt(&tables), collection_of(&[&tables, &tables])] {
        let out = to_sfnt(&bytes).unwrap();
        assert!(matches!(out, Cow::Borrowed(_)));
        assert_eq!(&*out, &bytes[..]);
    }
}

#[test]
fn a_woff_file_becomes_a_font_that_read_font_reads_the_same() {
    let tables = typical_tables("Inter", "Bold", 700, 0);
    let bytes = woff(&tables);
    let out = to_sfnt(&bytes).unwrap();
    assert!(matches!(out, Cow::Owned(_)));
    assert_eq!(&out[..4], &[0, 1, 0, 0]);
    assert_eq!(read_font(&out), read_font(&bytes));
}

#[test]
fn keeps_every_table_not_only_the_ones_read_font_wants() {
    let tables = [
        (*b"glyf", vec![9; 7]),
        (*b"head", vec![1, 2]),
        (*b"name", vec![3; 5]),
    ];
    let out = to_sfnt(&woff(&tables)).unwrap().into_owned();
    // Version, 3 tables: searchRange 32, entrySelector 1, rangeShift 16.
    assert_eq!(out[..12], [0, 1, 0, 0, 0, 3, 0, 32, 0, 1, 0, 16]);
    let mut offset = 12 + 16 * 3;
    for (index, (tag, data)) in tables.iter().enumerate() {
        let record = &out[12 + 16 * index..28 + 16 * index];
        assert_eq!(&record[..4], tag);
        assert_eq!(be32(record, 8) as usize, offset);
        assert_eq!(be32(record, 12) as usize, data.len());
        assert_eq!(&out[offset..offset + data.len()], &data[..]);
        offset += data.len().next_multiple_of(4);
        assert_eq!(offset % 4, 0);
    }
    assert_eq!(out.len(), offset);
}

#[test]
fn the_search_fields_follow_the_table_count() {
    let tables = |n: usize| -> Vec<Table> {
        (b'a'..)
            .take(n)
            .map(|last| ([b'a', b'a', b'a', last], vec![0]))
            .collect()
    };
    let header = |n: usize| to_sfnt(&woff(&tables(n))).unwrap()[..12].to_vec();
    assert_eq!(header(0), [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    assert_eq!(header(1), [0, 1, 0, 0, 0, 1, 0, 16, 0, 0, 0, 0]);
    assert_eq!(header(4), [0, 1, 0, 0, 0, 4, 0, 64, 0, 2, 0, 0]);
    assert_eq!(header(5), [0, 1, 0, 0, 0, 5, 0, 64, 0, 2, 0, 16]);
}

#[test]
fn copies_the_flavor_and_the_checksums_of_the_woff() {
    let mut bytes = woff(&[(*b"name", vec![1, 2, 3, 4])]);
    bytes[4..8].copy_from_slice(b"OTTO");
    bytes[44 + 16..44 + 20].copy_from_slice(&[0xde, 0xad, 0xbe, 0xef]);
    let out = to_sfnt(&bytes).unwrap();
    assert_eq!(&out[..4], b"OTTO");
    assert_eq!(&out[16..20], &[0xde, 0xad, 0xbe, 0xef]);
}

#[test]
fn inflates_the_compressed_tables() {
    let mut bytes = woff(&[(*b"name", NAME_TABLE_COMPRESSED.to_vec())]);
    let size = u32::try_from(NAME_TABLE_ORIGINAL_SIZE).unwrap();
    bytes[44 + 12..44 + 16].copy_from_slice(&size.to_be_bytes());
    let out = to_sfnt(&bytes).unwrap();
    assert_eq!(
        out.len(),
        12 + 16 + NAME_TABLE_ORIGINAL_SIZE.next_multiple_of(4)
    );
    assert_eq!(read_font(&out).unwrap().family, "Neutrif Pro");
}

#[test]
fn refuses_what_is_not_a_supported_font() {
    assert_eq!(to_sfnt(b"abc").err(), Some(FontError::Truncated));
    assert_eq!(to_sfnt(b"%PDF-1.4").err(), Some(FontError::UnknownFormat));
    assert_eq!(
        to_sfnt(b"wOF2abcdefgh").err(),
        Some(FontError::UnsupportedWoff2)
    );
}

#[test]
fn refuses_a_damaged_woff() {
    let bytes = woff(&[(*b"name", vec![1; 12]), (*b"head", vec![2; 8])]);
    for len in 0..bytes.len() {
        assert_eq!(
            to_sfnt(&bytes[..len]).err(),
            Some(FontError::Truncated),
            "{len}"
        );
    }
    let mut garbage = woff(&[(*b"name", vec![0xff; 20])]);
    garbage[44 + 12..44 + 16].copy_from_slice(&100u32.to_be_bytes());
    assert_eq!(
        to_sfnt(&garbage).err(),
        Some(FontError::InvalidCompressedData)
    );
    let mut absurd = woff(&[(*b"name", vec![0xff; 20])]);
    absurd[44 + 12..44 + 16].copy_from_slice(&u32::MAX.to_be_bytes());
    assert_eq!(
        to_sfnt(&absurd).err(),
        Some(FontError::InvalidCompressedData)
    );
}

#[test]
fn refuses_tables_that_add_up_to_more_than_the_limit() {
    // Two tables of 200 MB each: each under the limit, together over it.
    let mut bytes = woff(&[(*b"head", vec![1; 4]), (*b"name", vec![2; 4])]);
    for at in [44 + 12, 44 + 20 + 12] {
        bytes[at..at + 4].copy_from_slice(&200_000_000u32.to_be_bytes());
    }
    assert_eq!(
        to_sfnt(&bytes).err(),
        Some(FontError::InvalidCompressedData)
    );
}
