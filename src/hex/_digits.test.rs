// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn encodes_lower_and_upper() {
    assert_eq!(encode_with(&[0x00, 0xff, 0x1a], false), "00ff1a");
    assert_eq!(encode_with(&[0x00, 0xff, 0x1a], true), "00FF1A");
    assert_eq!(encode_with(&[], false), "");
}

#[test]
fn nibble_accepts_both_cases_and_rejects_the_rest() {
    assert_eq!(nibble(b'0'), Some(0));
    assert_eq!(nibble(b'9'), Some(9));
    assert_eq!(nibble(b'a'), Some(10));
    assert_eq!(nibble(b'F'), Some(15));
    assert_eq!(nibble(b'g'), None);
    assert_eq!(nibble(b'/'), None);
}

#[test]
fn decode_into_fills_the_slice() {
    let mut out = [0u8; 3];
    decode_into("00fF1a", &mut out).unwrap();
    assert_eq!(out, [0x00, 0xff, 0x1a]);
}

#[test]
fn decode_into_reports_the_first_invalid_character() {
    let mut out = [0u8; 2];
    assert_eq!(
        decode_into("0g00", &mut out),
        Err(DecodeError::InvalidChar {
            index: 1,
            found: 'g'
        })
    );
    assert_eq!(
        decode_into("00é0", &mut out),
        Err(DecodeError::InvalidChar {
            index: 2,
            found: 'é'
        })
    );
}
