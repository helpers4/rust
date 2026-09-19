// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn fills_an_exactly_sized_slice() {
    let mut buf = [0u8; 2];
    decode_to_slice("beEF", &mut buf).unwrap();
    assert_eq!(buf, [0xbe, 0xef]);
}

#[test]
fn odd_length_is_rejected_first() {
    assert_eq!(
        decode_to_slice("abc", &mut [0u8; 2]),
        Err(DecodeError::OddLength)
    );
}

#[test]
fn size_mismatch_is_rejected() {
    assert_eq!(
        decode_to_slice("beef", &mut [0u8; 3]),
        Err(DecodeError::InvalidLength {
            expected: 6,
            actual: 4
        })
    );
}

#[test]
fn invalid_character_is_rejected() {
    assert_eq!(
        decode_to_slice("bxef", &mut [0u8; 2]),
        Err(DecodeError::InvalidChar {
            index: 1,
            found: 'x'
        })
    );
}
