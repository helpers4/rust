// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn display_describes_each_variant() {
    assert_eq!(
        Base32DecodeError::InvalidLength { actual: 3 }.to_string(),
        "base32 string has 3 characters, expected a multiple of 8"
    );
    assert_eq!(
        Base32DecodeError::InvalidPadding.to_string(),
        "'=' padding must only appear at the end"
    );
    assert_eq!(
        Base32DecodeError::InvalidChar {
            index: 2,
            found: '!'
        }
        .to_string(),
        "invalid base32 character '!' at byte 2"
    );
}

#[test]
fn is_a_std_error() {
    let err: &dyn std::error::Error = &Base32DecodeError::InvalidPadding;
    assert!(err.source().is_none());
}
