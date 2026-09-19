// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

// Every `N` used here is a separate instantiation of the generic function, and coverage counts
// each one on its own: each is driven through both the success and the error path.

#[test]
fn decodes_into_an_array_of_the_requested_size() {
    assert_eq!(decode_array::<3>("00ff1a").unwrap(), [0x00, 0xff, 0x1a]);
    assert_eq!(
        decode_array::<3>("00ff"),
        Err(DecodeError::InvalidLength {
            expected: 6,
            actual: 4
        })
    );
}

#[test]
fn wrong_length_is_rejected() {
    assert_eq!(
        decode_array::<4>("deadbeef").unwrap(),
        [0xde, 0xad, 0xbe, 0xef]
    );
    assert_eq!(
        decode_array::<4>("dead"),
        Err(DecodeError::InvalidLength {
            expected: 8,
            actual: 4
        })
    );
}

#[test]
fn zero_sized_array_only_accepts_the_empty_string() {
    assert_eq!(decode_array::<0>("").unwrap(), [0u8; 0]);
    assert!(decode_array::<0>("00").is_err());
}

#[test]
fn a_32_byte_key_round_trips() {
    let hex = "00".repeat(31) + "ff";
    let key: [u8; 32] = decode_array(&hex).unwrap();
    assert_eq!(key[31], 0xff);
    assert!(decode_array::<32>("00").is_err());
}
