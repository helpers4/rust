// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn decodes_either_case() {
    assert_eq!(decode("00fF1a").unwrap(), [0x00, 0xff, 0x1a]);
    assert_eq!(decode("").unwrap(), Vec::<u8>::new());
}

#[test]
fn odd_length_is_rejected() {
    assert_eq!(decode("abc"), Err(DecodeError::OddLength));
}

#[test]
fn invalid_character_is_reported_with_its_position() {
    assert_eq!(
        decode("zz"),
        Err(DecodeError::InvalidChar {
            index: 0,
            found: 'z'
        })
    );
    assert_eq!(
        decode("0x12"),
        Err(DecodeError::InvalidChar {
            index: 1,
            found: 'x'
        })
    );
}
