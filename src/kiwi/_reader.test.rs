// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn var_uint_reads_leb128() {
    assert_eq!(Reader::new(&[0x00]).var_uint(), Ok(0));
    assert_eq!(Reader::new(&[0x7f]).var_uint(), Ok(127));
    assert_eq!(Reader::new(&[0x80, 0x01]).var_uint(), Ok(128));
    assert_eq!(
        Reader::new(&[0xff, 0xff, 0xff, 0xff, 0x0f]).var_uint(),
        Ok(u32::MAX)
    );
}

#[test]
fn var_int_reads_zigzag() {
    assert_eq!(Reader::new(&[0x00]).var_int(), Ok(0));
    assert_eq!(Reader::new(&[0x01]).var_int(), Ok(-1));
    assert_eq!(Reader::new(&[0x02]).var_int(), Ok(1));
    assert_eq!(Reader::new(&[0x03]).var_int(), Ok(-2));
}

#[test]
fn var_uint64_reads_seven_bit_groups_then_a_full_ninth_byte() {
    assert_eq!(Reader::new(&[0x00]).var_uint64(), Ok(0));
    assert_eq!(Reader::new(&[0xac, 0x02]).var_uint64(), Ok(300));
    let mut max = vec![0xff; 8];
    max.push(0xff);
    assert_eq!(Reader::new(&max).var_uint64(), Ok(u64::MAX));
    // The ninth byte is not a continuation byte: its high bit is data.
    let mut ninth = vec![0x80; 8];
    ninth.push(0x80);
    assert_eq!(Reader::new(&ninth).var_uint64(), Ok(0x80 << 56));
}

#[test]
fn var_int64_reads_zigzag() {
    assert_eq!(Reader::new(&[0x00]).var_int64(), Ok(0));
    assert_eq!(Reader::new(&[0x01]).var_int64(), Ok(-1));
    assert_eq!(Reader::new(&[0x02]).var_int64(), Ok(1));
    let mut min = vec![0xff; 8];
    min.push(0xff);
    assert_eq!(Reader::new(&min).var_int64(), Ok(i64::MIN));
}

#[test]
fn var_float_reads_kiwi_floats() {
    assert_eq!(Reader::new(&[0x00]).var_float().map(f32::to_bits), Ok(0));
    for value in [1.0f32, -2.5, 3.5, 1e-20] {
        let bytes = value.to_bits().rotate_right(23).to_le_bytes();
        assert_eq!(Reader::new(&bytes).var_float(), Ok(value));
    }
}

#[test]
fn string_reads_null_terminated_utf8() {
    let mut r = Reader::new(b"caf\xc3\xa9\0next\0");
    assert_eq!(r.string().as_deref(), Ok("café"));
    assert_eq!(r.string().as_deref(), Ok("next"));
}

#[test]
fn reports_where_a_read_fell_short() {
    let mut r = Reader::new(&[0x01]);
    assert_eq!(r.byte(), Ok(1));
    assert_eq!(r.byte(), Err(KiwiError::UnexpectedEof { index: 1 }));
    assert_eq!(
        Reader::new(&[0x80]).var_uint(),
        Err(KiwiError::UnexpectedEof { index: 0 })
    );
    assert_eq!(
        Reader::new(&[0x80]).var_int(),
        Err(KiwiError::UnexpectedEof { index: 0 })
    );
    assert_eq!(
        Reader::new(&[0x80, 0x80]).var_uint64(),
        Err(KiwiError::UnexpectedEof { index: 2 })
    );
    assert_eq!(
        Reader::new(&[0x01, 0x02]).var_float(),
        Err(KiwiError::UnexpectedEof { index: 1 })
    );
    assert_eq!(
        Reader::new(&[1, 2]).take(3),
        Err(KiwiError::UnexpectedEof { index: 0 })
    );
}

#[test]
fn reports_a_bad_string_where_it_starts() {
    let mut r = Reader::new(&[0x00, 0xff, 0x00]);
    assert_eq!(r.string().as_deref(), Ok(""));
    assert_eq!(r.string(), Err(KiwiError::InvalidString { index: 1 }));
    assert_eq!(
        Reader::new(b"no end").string(),
        Err(KiwiError::InvalidString { index: 0 })
    );
}

#[test]
fn counts_the_bytes_left() {
    let mut r = Reader::new(&[1, 2, 3]);
    assert_eq!(r.remaining(), 3);
    assert_eq!(r.take(2), Ok(&[1u8, 2][..]));
    assert_eq!(r.remaining(), 1);
}
