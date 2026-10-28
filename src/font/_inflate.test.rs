// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::super::_test_fixtures::*;
use super::*;

/// Writes bits least significant first, like DEFLATE.
#[derive(Default)]
struct BitWriter {
    bytes: Vec<u8>,
    used: u32,
}

impl BitWriter {
    fn put(&mut self, value: u32, count: u32) -> &mut Self {
        for i in 0..count {
            if self.used == 0 {
                self.bytes.push(0);
            }
            let bit = u8::try_from((value >> i) & 1).unwrap_or(0);
            if let Some(last) = self.bytes.last_mut() {
                *last |= bit << self.used;
            }
            self.used = (self.used + 1) % 8;
        }
        self
    }

    /// A Huffman code: its bits go most significant first.
    fn code(&mut self, code: u32, length: u32) -> &mut Self {
        for i in (0..length).rev() {
            self.put((code >> i) & 1, 1);
        }
        self
    }

    /// The bytes, as a zlib stream.
    fn zlib(&self) -> Vec<u8> {
        let mut out = vec![0x78, 0x01];
        out.extend(&self.bytes);
        out
    }
}

fn zlib(deflate: &[u8]) -> Vec<u8> {
    [&[0x78, 0x01][..], deflate].concat()
}

#[test]
fn inflates_a_stored_block() {
    assert_eq!(zlib_decompress(STORED, 11), Some(b"stored data".to_vec()));
}

#[test]
fn inflates_a_fixed_huffman_block_with_matches() {
    assert_eq!(
        zlib_decompress(FIXED, 17),
        Some(b"hello hello hello".to_vec())
    );
}

#[test]
fn inflates_a_dynamic_huffman_block() {
    let text = dynamic_text();
    assert_eq!(zlib_decompress(DYNAMIC, text.len()), Some(text));
}

#[test]
fn inflates_several_blocks() {
    assert_eq!(
        zlib_decompress(MULTI, 51),
        Some(b"first block, first block. second block second block".to_vec())
    );
}

#[test]
fn inflates_a_compressed_name_table() {
    let table = zlib_decompress(NAME_TABLE_COMPRESSED, NAME_TABLE_ORIGINAL_SIZE).unwrap();
    assert_eq!(table.len(), NAME_TABLE_ORIGINAL_SIZE);
    assert_eq!(&table[..2], [0, 0]);
}

#[test]
fn a_truncated_stream_never_inflates() {
    let text_len = dynamic_text().len();
    for (stream, size) in [
        (STORED, 11),
        (FIXED, 17),
        (DYNAMIC, text_len),
        (MULTI, 51),
        (NAME_TABLE_COMPRESSED, NAME_TABLE_ORIGINAL_SIZE),
    ] {
        for len in 0..stream.len() - 4 {
            assert_eq!(
                zlib_decompress(&stream[..len], size),
                None,
                "{len} of {}",
                stream.len()
            );
        }
    }
}

#[test]
fn the_size_must_be_exact() {
    assert_eq!(zlib_decompress(FIXED, 18), None);
    assert_eq!(zlib_decompress(FIXED, 16), None);
    assert_eq!(zlib_decompress(FIXED, 10), None);
    assert_eq!(zlib_decompress(STORED, 5), None);
    assert_eq!(zlib_decompress(STORED, 12), None);
}

#[test]
fn rejects_a_bad_header() {
    assert_eq!(zlib_decompress(&[], 0), None);
    assert_eq!(zlib_decompress(&[0x78], 0), None);
    // Not deflate (compression method 9).
    assert_eq!(zlib_decompress(&[0x79, 0x01], 0), None);
    // A header checksum that does not divide by 31.
    assert_eq!(zlib_decompress(&[0x78, 0x00], 0), None);
    // A preset dictionary.
    assert_eq!(zlib_decompress(&[0x78, 0xbb], 0), None);
}

#[test]
fn rejects_the_reserved_block_type() {
    assert_eq!(zlib_decompress(&zlib(&[0x07]), 0), None);
}

#[test]
fn rejects_a_stored_block_with_a_wrong_complement() {
    let bad = zlib(&[0x01, 0x01, 0x00, 0x00, 0x00, b'x']);
    assert_eq!(zlib_decompress(&bad, 1), None);
}

#[test]
fn rejects_a_stored_block_longer_than_its_data() {
    let short = zlib(&[0x01, 0x05, 0x00, 0xfa, 0xff, b'x']);
    assert_eq!(zlib_decompress(&short, 5), None);
}

#[test]
fn rejects_a_literal_past_the_size() {
    // A fixed block of one literal `a`, then the end of block, for a size of zero.
    let mut bits = BitWriter::default();
    bits.put(1, 1)
        .put(1, 2)
        .code(0x30 + u32::from(b'a'), 8)
        .code(0, 7);
    assert_eq!(zlib_decompress(&bits.zlib(), 1), Some(b"a".to_vec()));
    assert_eq!(zlib_decompress(&bits.zlib(), 0), None);
}

#[test]
fn rejects_a_match_before_the_start() {
    // Length 3 (symbol 257), distance 1 (symbol 0), with nothing written yet.
    let mut bits = BitWriter::default();
    bits.put(1, 1).put(1, 2).code(1, 7).code(0, 5).code(0, 7);
    assert_eq!(zlib_decompress(&bits.zlib(), 3), None);
}

#[test]
fn rejects_a_length_or_distance_symbol_that_does_not_exist() {
    // Literal/length symbol 286 (a fixed 8-bit code 0xc0 + 6).
    let mut length = BitWriter::default();
    length.put(1, 1).put(1, 2).code(0xc0 + 6, 8);
    assert_eq!(zlib_decompress(&length.zlib(), 3), None);
    // Distance code 30, which the fixed code leaves unassigned.
    let mut distance = BitWriter::default();
    distance
        .put(1, 1)
        .put(1, 2)
        .code(0x30 + u32::from(b'a'), 8)
        .code(1, 7)
        .code(30, 5)
        .put(0xffff, 16);
    assert_eq!(zlib_decompress(&distance.zlib(), 4), None);
}

#[test]
fn rejects_a_match_past_the_size() {
    let mut bits = BitWriter::default();
    bits.put(1, 1)
        .put(1, 2)
        .code(0x30 + u32::from(b'a'), 8)
        .code(1, 7)
        .code(0, 5)
        .code(0, 7);
    assert_eq!(zlib_decompress(&bits.zlib(), 4), Some(b"aaaa".to_vec()));
    assert_eq!(zlib_decompress(&bits.zlib(), 3), None);
}

/// The start of a dynamic block: header counts, then the code length code lengths.
fn dynamic_start(literals: u32, distances: u32, lengths: &[u32]) -> BitWriter {
    let mut bits = BitWriter::default();
    bits.put(1, 1).put(2, 2);
    bits.put(literals - 257, 5).put(distances - 1, 5);
    bits.put(u32::try_from(lengths.len()).unwrap_or(4) - 4, 4);
    for &length in lengths {
        bits.put(length, 3);
    }
    bits
}

#[test]
fn rejects_too_many_literal_or_distance_codes() {
    assert_eq!(
        zlib_decompress(&dynamic_start(287, 1, &[0; 4]).zlib(), 0),
        None
    );
    assert_eq!(
        zlib_decompress(&dynamic_start(257, 31, &[0; 4]).zlib(), 0),
        None
    );
}

#[test]
fn rejects_an_over_subscribed_code_length_code() {
    let mut bits = dynamic_start(257, 1, &[1; 19]);
    bits.put(0, 8);
    assert_eq!(zlib_decompress(&bits.zlib(), 0), None);
}

#[test]
fn rejects_a_repeat_with_nothing_to_repeat() {
    // Only symbol 16 has a code (`0`), and it comes first.
    let mut bits = dynamic_start(257, 1, &[1, 0, 0, 0]);
    bits.code(0, 1).put(0, 2);
    assert_eq!(zlib_decompress(&bits.zlib(), 0), None);
}

#[test]
fn reads_a_run_of_zeros_of_the_short_kind() {
    // Only symbol 17 has a code: a run of 3 + 7 zeros, then the data ends.
    let mut bits = dynamic_start(257, 1, &[0, 1, 0, 0]);
    bits.code(0, 1).put(7, 3);
    assert_eq!(zlib_decompress(&bits.zlib(), 0), None);
}

#[test]
fn rejects_a_repeat_whose_count_is_missing() {
    // Code lengths: symbol 0 has the code `0`, symbol 16 the code `10`. The data ends exactly
    // after the `16`, so its two repeat bits are missing.
    let mut bits = dynamic_start(257, 1, &[2, 0, 0, 1]);
    bits.code(0, 1).code(0b10, 2);
    assert_eq!(bits.bytes.len(), 4);
    assert_eq!(zlib_decompress(&bits.zlib(), 0), None);
}

#[test]
fn rejects_a_block_header_cut_after_its_first_bit() {
    // Three non-last empty fixed blocks, the first with a 9-bit literal: 39 bits, so the next
    // block's final-block bit is the last bit of the data and its type bits are missing.
    let mut bits = BitWriter::default();
    bits.put(0, 1).put(1, 2).code(0x190, 9).code(0, 7);
    for _ in 0..2 {
        bits.put(0, 1).put(1, 2).code(0, 7);
    }
    assert_eq!(bits.bytes.len(), 5);
    assert_eq!(zlib_decompress(&bits.zlib(), 1), None);
}

#[test]
fn rejects_repeats_that_overflow_the_lengths() {
    // Only symbol 18 has a code: two runs of 138 zeros, for 258 lengths.
    let mut bits = dynamic_start(257, 1, &[0, 0, 1, 0]);
    bits.code(0, 1).put(127, 7).code(0, 1).put(127, 7);
    assert_eq!(zlib_decompress(&bits.zlib(), 0), None);
}

#[test]
fn rejects_a_block_without_an_end_of_block_code() {
    // 138 + 120 zeros: every length is zero, so symbol 256 has no code.
    let mut bits = dynamic_start(257, 1, &[0, 0, 1, 0]);
    bits.code(0, 1).put(127, 7).code(0, 1).put(109, 7);
    assert_eq!(zlib_decompress(&bits.zlib(), 0), None);
}

#[test]
fn reads_the_runs_of_zeros_and_repeats_of_a_dynamic_block() {
    // The fixture uses symbols 16, 17 and 18; inflating it exercises them.
    let text = dynamic_text();
    assert_eq!(
        zlib_decompress(DYNAMIC, text.len()).map(|t| t.len()),
        Some(text.len())
    );
}
