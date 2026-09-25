// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn rfc4648_test_vectors() {
    assert_eq!(base32_decode(""), Ok(Vec::new()));
    assert_eq!(base32_decode("MY======"), Ok(b"f".to_vec()));
    assert_eq!(base32_decode("MZXQ===="), Ok(b"fo".to_vec()));
    assert_eq!(base32_decode("MZXW6==="), Ok(b"foo".to_vec()));
    assert_eq!(base32_decode("MZXW6YQ="), Ok(b"foob".to_vec()));
    assert_eq!(base32_decode("MZXW6YTB"), Ok(b"fooba".to_vec()));
    assert_eq!(base32_decode("MZXW6YTBOI======"), Ok(b"foobar".to_vec()));
}

#[test]
fn length_not_a_multiple_of_eight_is_an_error() {
    assert_eq!(
        base32_decode("MZXW6YTBOI"),
        Err(Base32DecodeError::InvalidLength { actual: 10 })
    );
}

#[test]
fn padding_before_the_end_is_an_error() {
    assert_eq!(
        base32_decode("M=XQ===="),
        Err(Base32DecodeError::InvalidPadding)
    );
}

#[test]
fn seven_padding_characters_is_an_error() {
    // 8 chars, last 7 are '=': take_while caps the counted padding at 6, so the un-stripped
    // remainder still contains a '=' and is rejected as InvalidPadding rather than miscounted.
    assert_eq!(
        base32_decode("M======="),
        Err(Base32DecodeError::InvalidPadding)
    );
}

#[test]
fn a_character_outside_the_alphabet_is_an_error() {
    assert_eq!(
        base32_decode("M1XQ===="),
        Err(Base32DecodeError::InvalidChar {
            index: 1,
            found: '1'
        })
    );
}

#[test]
fn a_non_ascii_character_reports_the_whole_codepoint() {
    assert_eq!(
        base32_decode("MéXXXXX"), // 8 bytes: 'M' + 2-byte 'é' + 5 ASCII
        Err(Base32DecodeError::InvalidChar {
            index: 1,
            found: 'é'
        })
    );
}
