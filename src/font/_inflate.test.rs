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

/// The canonical Huffman codes of a set of code lengths, as `(code, length)` per symbol.
fn canonical(lengths: &[u32]) -> Vec<(u32, u32)> {
    let longest = lengths.iter().copied().max().unwrap_or(0);
    let mut code = 0;
    let mut out = vec![(0, 0); lengths.len()];
    for length in 1..=longest {
        for (symbol, &known) in lengths.iter().enumerate() {
            if known == length {
                out[symbol] = (code, length);
                code += 1;
            }
        }
        code <<= 1;
    }
    out
}

/// One entry of the code lengths of a dynamic block, written with the code length code.
enum Op {
    /// The nonzero length `value` (its own symbol).
    Value,
    /// A run of 11 to 138 zeros (symbol 18).
    Zeros(u32),
    /// A run of 3 to 10 zeros (symbol 17).
    ShortZeros(u32),
    /// 3 to 6 copies of the previous length (symbol 16).
    Repeat(u32),
}

/// A dynamic block header (`hlit` literal codes, `hdist` distance codes), with the lengths
/// written by `ops` and the nonzero length `value`. The block data follows through `bits`.
fn dynamic_block(hlit: u32, hdist: u32, value: usize, ops: &[Op]) -> BitWriter {
    let mut lengths = [0u32; 19];
    lengths[0] = 2;
    lengths[value] = 2;
    lengths[18] = 2;
    lengths[16] = 3;
    lengths[17] = 3;
    let codes = canonical(&lengths);
    let order = [
        16usize, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
    ];
    let used = order
        .iter()
        .rposition(|&symbol| lengths[symbol] != 0)
        .unwrap_or(3)
        + 1;

    let mut bits = BitWriter::default();
    bits.put(1, 1)
        .put(2, 2)
        .put(hlit - 257, 5)
        .put(hdist - 1, 5);
    bits.put(u32::try_from(used).unwrap_or(4) - 4, 4);
    for &symbol in &order[..used] {
        bits.put(lengths[symbol], 3);
    }
    for op in ops {
        let (symbol, extra, extra_bits) = match op {
            Op::Value => (value, 0, 0),
            Op::Zeros(n) => (18, n - 11, 7),
            Op::ShortZeros(n) => (17, n - 3, 3),
            Op::Repeat(n) => (16, n - 3, 2),
        };
        let (code, length) = codes[symbol];
        bits.code(code, length).put(extra, extra_bits);
    }
    bits
}

/// The ops of a block with the literal codes `0` and `256` and the distance code `0`, each of
/// length 1, and `hlit` literal and `hdist` distance entries in all.
fn two_literal_ops(hlit: u32, hdist: u32) -> Vec<Op> {
    let mut ops = vec![Op::Value, Op::Zeros(138), Op::Zeros(117), Op::Value];
    if hlit > 257 {
        ops.push(Op::Zeros(hlit - 257));
    }
    ops.push(Op::Value);
    if hdist > 1 {
        ops.push(Op::Zeros(hdist - 1));
    }
    ops
}

fn two_literal_block(hlit: u32, hdist: u32) -> Vec<u8> {
    let mut bits = dynamic_block(hlit, hdist, 1, &two_literal_ops(hlit, hdist));
    // Literal 0 is the code `0`, the end of block the code `1`.
    bits.code(0, 1).code(1, 1);
    bits.zlib()
}

#[test]
fn accepts_the_largest_literal_and_distance_counts() {
    assert_eq!(
        zlib_decompress(&two_literal_block(286, 30), 1),
        Some(vec![0])
    );
    assert_eq!(
        zlib_decompress(&two_literal_block(257, 1), 1),
        Some(vec![0])
    );
}

#[test]
fn rejects_one_literal_or_distance_code_too_many_in_an_otherwise_valid_block() {
    assert_eq!(zlib_decompress(&two_literal_block(287, 30), 1), None);
    assert_eq!(zlib_decompress(&two_literal_block(257, 31), 1), None);
    assert_eq!(zlib_decompress(&two_literal_block(287, 31), 1), None);
}

#[test]
fn reads_repeats_of_the_previous_length_and_short_runs_of_zeros() {
    // Lengths 3 for symbols 0 to 5 (a 3, then 5 copies), 250 zeros, 3 for the end of block, then
    // a distance code of length 3 and 29 zeros (7 + 22).
    let ops = [
        Op::Value,
        Op::Repeat(5),
        Op::Zeros(138),
        Op::Zeros(112),
        Op::Value,
        Op::Value,
        Op::ShortZeros(7),
        Op::Zeros(22),
    ];
    let mut bits = dynamic_block(257, 30, 3, &ops);
    // Seven codes of length 3: symbols 0 to 5 are `000` to `101`, the end of block is `110`.
    bits.code(0b101, 3).code(0b010, 3).code(0b110, 3);
    assert_eq!(zlib_decompress(&bits.zlib(), 2), Some(vec![5, 2]));
}

#[test]
fn a_stream_may_end_exactly_on_its_last_bit() {
    // Six 9-bit literals (144 to 149) between a 3-bit header and a 7-bit end of block: 64 bits.
    let mut bits = BitWriter::default();
    bits.put(1, 1).put(1, 2);
    for literal in 0..6 {
        bits.code(0x190 + literal, 9);
    }
    bits.code(0, 7);
    assert_eq!(bits.bytes.len(), 8);
    assert_eq!(
        zlib_decompress(&bits.zlib(), 6),
        Some(vec![144, 145, 146, 147, 148, 149])
    );
}

#[test]
fn stored_blocks_count_what_the_earlier_blocks_wrote() {
    let stream = zlib(&[
        0x00, 2, 0, 0xfd, 0xff, b'a', b'b', // not the last block
        0x01, 2, 0, 0xfd, 0xff, b'c', b'd',
    ]);
    assert_eq!(zlib_decompress(&stream, 4), Some(b"abcd".to_vec()));
    assert_eq!(zlib_decompress(&stream, 3), None);
}

#[test]
fn each_header_fault_alone_is_enough_to_refuse_a_valid_stream() {
    let body = [0x01, 1, 0, 0xfe, 0xff, b'x'];
    let with = |cmf: u8, flg: u8| [&[cmf, flg][..], &body].concat();
    assert_eq!(zlib_decompress(&with(0x78, 0x01), 1), Some(b"x".to_vec()));
    // Another compression method, with a header checksum that is right.
    assert_eq!(zlib_decompress(&with(0x79, 0x18), 1), None);
    // The right method, with a checksum that is wrong.
    assert_eq!(zlib_decompress(&with(0x78, 0x00), 1), None);
    // The right method and checksum, with a preset dictionary.
    assert_eq!(zlib_decompress(&with(0x78, 0xbb), 1), None);
}

#[test]
fn a_stored_block_may_not_pass_the_limit_together_with_what_is_already_written() {
    let block = [2, 0, 0xfd, 0xff, b'c', b'd'];
    let mut out = vec![b'a', b'b'];
    assert_eq!(stored(&mut Bits::new(&block), &mut out, 3), None);
    assert_eq!(out, b"ab");
    assert_eq!(stored(&mut Bits::new(&block), &mut out, 4), Some(()));
    assert_eq!(out, b"abcd");
    // A block that fits alone but not after one byte.
    let mut one = vec![b'a'];
    assert_eq!(stored(&mut Bits::new(&block), &mut one, 2), None);
}
