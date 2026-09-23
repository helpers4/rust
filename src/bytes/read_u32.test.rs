// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn reads_big_endian() {
    let bytes = [0x12, 0x34, 0x56, 0x78];
    assert_eq!(read_u32(&bytes, 0, Endian::Big), Some(0x1234_5678));
}

#[test]
fn reads_little_endian() {
    let bytes = [0x12, 0x34, 0x56, 0x78];
    assert_eq!(read_u32(&bytes, 0, Endian::Little), Some(0x7856_3412));
}

#[test]
fn reads_at_an_offset() {
    let mut bytes = vec![0xff; 3];
    bytes.extend_from_slice(&32u32.to_be_bytes());
    assert_eq!(read_u32(&bytes, 3, Endian::Big), Some(32));
}

#[test]
fn too_few_bytes_gives_none() {
    let bytes = [0u8; 4];
    assert_eq!(read_u32(&bytes, 1, Endian::Big), None);
    assert_eq!(read_u32(&bytes[..3], 0, Endian::Little), None);
    assert_eq!(read_u32(&[], 0, Endian::Big), None);
}

#[test]
fn an_offset_that_overflows_gives_none() {
    assert_eq!(read_u32(&[0u8; 4], usize::MAX, Endian::Big), None);
}
