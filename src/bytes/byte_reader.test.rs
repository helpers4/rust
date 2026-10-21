// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn starts_at_zero_and_tracks_the_position() {
    let mut reader = ByteReader::new(&[1, 2, 3]);
    assert_eq!((reader.position(), reader.remaining()), (0, 3));
    assert_eq!(reader.read_u8(), Some(1));
    assert_eq!((reader.position(), reader.remaining()), (1, 2));
}

#[test]
fn read_u8_stops_at_the_end() {
    let mut reader = ByteReader::new(&[9]);
    assert_eq!(reader.read_u8(), Some(9));
    assert_eq!(reader.read_u8(), None);
    assert_eq!(reader.position(), 1);
}

#[test]
fn read_bytes_borrows_and_refuses_too_many() {
    let data = [1, 2, 3, 4];
    let mut reader = ByteReader::new(&data);
    assert_eq!(reader.read_bytes(0), Some(&[][..]));
    assert_eq!(reader.read_bytes(3), Some(&data[..3]));
    assert_eq!(reader.read_bytes(2), None);
    assert_eq!(reader.read_bytes(usize::MAX), None);
    assert_eq!(reader.position(), 3);
}

#[test]
fn reads_fixed_size_integers_in_both_orders() {
    let data = [0x12, 0x34, 0x12, 0x34, 0x56, 0x78, 1, 2, 3, 4, 5, 6, 7, 8];
    let mut reader = ByteReader::new(&data);
    assert_eq!(reader.read_u16(Endian::Big), Some(0x1234));
    assert_eq!(reader.read_u32(Endian::Little), Some(0x7856_3412));
    assert_eq!(reader.read_u64(Endian::Big), Some(0x0102_0304_0506_0708));
    assert_eq!(reader.remaining(), 0);
}

#[test]
fn fixed_size_reads_leave_the_position_on_failure() {
    let mut reader = ByteReader::new(&[1]);
    assert_eq!(reader.read_u16(Endian::Big), None);
    assert_eq!(reader.read_u32(Endian::Big), None);
    assert_eq!(reader.read_u64(Endian::Big), None);
    assert_eq!(reader.read_i16(Endian::Big), None);
    assert_eq!(reader.read_i32(Endian::Big), None);
    assert_eq!(reader.read_i64(Endian::Big), None);
    assert_eq!(reader.read_f32(Endian::Big), None);
    assert_eq!(reader.read_f64(Endian::Big), None);
    assert_eq!(reader.position(), 0);
}

#[test]
fn reads_signed_integers_and_floats() {
    let mut data = Vec::new();
    data.extend_from_slice(&(-2i16).to_le_bytes());
    data.extend_from_slice(&i32::MIN.to_be_bytes());
    data.extend_from_slice(&(-3i64).to_le_bytes());
    data.extend_from_slice(&1.5f32.to_be_bytes());
    data.extend_from_slice(&f64::MIN_POSITIVE.to_le_bytes());
    let mut reader = ByteReader::new(&data);
    assert_eq!(reader.read_i16(Endian::Little), Some(-2));
    assert_eq!(reader.read_i32(Endian::Big), Some(i32::MIN));
    assert_eq!(reader.read_i64(Endian::Little), Some(-3));
    assert_eq!(reader.read_f32(Endian::Big), Some(1.5));
    assert_eq!(reader.read_f64(Endian::Little), Some(f64::MIN_POSITIVE));
}

#[test]
fn reads_var_u32() {
    for (bytes, value) in [
        (&[0x00][..], 0),
        (&[0x7f], 127),
        (&[0x80, 0x01], 128),
        (&[0xac, 0x02], 300),
        (&[0xff, 0xff, 0xff, 0xff, 0x0f], u32::MAX),
        (&[0x80, 0x00], 0), // overlong
    ] {
        let mut reader = ByteReader::new(bytes);
        assert_eq!(reader.read_var_u32(), Some(value), "{bytes:02x?}");
        assert_eq!(reader.remaining(), 0);
    }
}

#[test]
fn var_u32_rejects_truncation_and_overflow_without_moving() {
    for bytes in [
        &[][..],
        &[0x80],
        &[0xff, 0xff, 0xff, 0xff],
        &[0xff, 0xff, 0xff, 0xff, 0x10],
        &[0xff, 0xff, 0xff, 0xff, 0x8f, 0x00],
    ] {
        let mut reader = ByteReader::new(bytes);
        assert_eq!(reader.read_var_u32(), None, "{bytes:02x?}");
        assert_eq!(reader.position(), 0);
    }
}

#[test]
fn reads_var_u64_up_to_ten_bytes() {
    let max = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01];
    assert_eq!(ByteReader::new(&max).read_var_u64(), Some(u64::MAX));
    assert_eq!(ByteReader::new(&[0xac, 0x02]).read_var_u64(), Some(300));
}

#[test]
fn var_u64_rejects_truncation_and_overflow() {
    let overflow = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x02];
    let too_long = [
        0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x81, 0x00,
    ];
    assert_eq!(ByteReader::new(&overflow).read_var_u64(), None);
    assert_eq!(ByteReader::new(&too_long).read_var_u64(), None);
    assert_eq!(ByteReader::new(&[0xff; 3]).read_var_u64(), None);
}

#[test]
fn reads_zigzag_varints() {
    let mut reader = ByteReader::new(&[
        0x00, 0x01, 0x02, 0x03, 0xfe, 0xff, 0xff, 0xff, 0x0f, 0xff, 0xff, 0xff, 0xff, 0x0f,
    ]);
    assert_eq!(reader.read_var_i32(), Some(0));
    assert_eq!(reader.read_var_i32(), Some(-1));
    assert_eq!(reader.read_var_i32(), Some(1));
    assert_eq!(reader.read_var_i64(), Some(-2));
    assert_eq!(reader.read_var_i32(), Some(i32::MAX));
    assert_eq!(reader.read_var_i32(), Some(i32::MIN));
    assert_eq!(reader.read_var_i32(), None);
    assert_eq!(reader.read_var_i64(), None);
}

#[test]
fn reads_nul_terminated_strings() {
    let mut reader = ByteReader::new("é\0\0rest".as_bytes());
    assert_eq!(reader.read_cstr(), Some("é"));
    assert_eq!(reader.read_cstr(), Some(""));
    assert_eq!(reader.position(), 4);
    assert_eq!(reader.read_cstr(), None); // no terminator
    assert_eq!(reader.position(), 4);
}

#[test]
fn read_cstr_rejects_invalid_utf8_without_moving() {
    let mut reader = ByteReader::new(&[0xff, 0x00]);
    assert_eq!(reader.read_cstr(), None);
    assert_eq!(reader.position(), 0);
}

#[test]
fn read_cstr_at_the_end_returns_none() {
    let mut reader = ByteReader::new(&[0]);
    assert_eq!(reader.read_cstr(), Some(""));
    assert_eq!(reader.read_cstr(), None);
}
