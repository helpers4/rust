// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn empty_input_is_empty() {
    assert_eq!(base64_decode(""), Ok(Vec::new()));
}

#[test]
fn one_byte_with_two_padding_characters() {
    assert_eq!(base64_decode("TQ=="), Ok(b"M".to_vec()));
}

#[test]
fn two_bytes_with_one_padding_character() {
    assert_eq!(base64_decode("TWE="), Ok(b"Ma".to_vec()));
}

#[test]
fn three_bytes_need_no_padding() {
    assert_eq!(base64_decode("TWFu"), Ok(b"Man".to_vec()));
}

#[test]
fn more_than_one_group() {
    assert_eq!(
        base64_decode("YW55IGNhcm5hbCBwbGVhcw=="),
        Ok(b"any carnal pleas".to_vec())
    );
}

#[test]
fn length_not_a_multiple_of_four_is_an_error() {
    assert_eq!(
        base64_decode("TWFu!"),
        Err(Base64DecodeError::InvalidLength { actual: 5 })
    );
}

#[test]
fn padding_before_the_end_is_an_error() {
    assert_eq!(
        base64_decode("T=Fu"),
        Err(Base64DecodeError::InvalidPadding)
    );
}

#[test]
fn three_padding_characters_is_an_error() {
    // 4 chars, last 3 are '=': take_while caps the counted padding at 2, so the un-stripped
    // remainder still contains a '=' and is rejected as InvalidPadding rather than miscounted.
    assert_eq!(
        base64_decode("T==="),
        Err(Base64DecodeError::InvalidPadding)
    );
}

#[test]
fn a_character_outside_the_alphabet_is_an_error() {
    assert_eq!(
        base64_decode("TW!u"),
        Err(Base64DecodeError::InvalidChar {
            index: 2,
            found: '!'
        })
    );
}

#[test]
fn a_non_ascii_character_reports_the_whole_codepoint() {
    assert_eq!(
        base64_decode("TWé"),
        Err(Base64DecodeError::InvalidChar {
            index: 2,
            found: 'é'
        })
    );
}
