// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn reads_big_endian() {
    let bytes = [0, 0, 0, 0, 0, 0, 1, 0];
    assert_eq!(read_u64(&bytes, 0, Endian::Big), Some(256));
}

#[test]
fn reads_little_endian() {
    let bytes = [0, 0, 0, 0, 0, 0, 1, 0];
    assert_eq!(
        read_u64(&bytes, 0, Endian::Little),
        Some(0x0001_0000_0000_0000)
    );
}

#[test]
fn reads_at_an_offset() {
    let mut bytes = vec![0xff; 3];
    bytes.extend_from_slice(&64u64.to_be_bytes());
    assert_eq!(read_u64(&bytes, 3, Endian::Big), Some(64));
}

#[test]
fn too_few_bytes_gives_none() {
    let bytes = [0u8; 8];
    assert_eq!(read_u64(&bytes, 1, Endian::Big), None);
    assert_eq!(read_u64(&bytes[..7], 0, Endian::Little), None);
    assert_eq!(read_u64(&[], 0, Endian::Big), None);
}

#[test]
fn an_offset_that_overflows_gives_none() {
    assert_eq!(read_u64(&[0u8; 8], usize::MAX, Endian::Big), None);
}
