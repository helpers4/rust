// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn display_describes_each_variant() {
    assert_eq!(
        DecodeError::OddLength.to_string(),
        "hex string has an odd length"
    );
    assert_eq!(
        DecodeError::InvalidLength {
            expected: 8,
            actual: 6
        }
        .to_string(),
        "hex string has 6 characters, expected 8"
    );
    assert_eq!(
        DecodeError::InvalidChar {
            index: 3,
            found: 'g'
        }
        .to_string(),
        "invalid hex character 'g' at byte 3"
    );
}

#[test]
fn is_a_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(DecodeError::OddLength);
    assert!(err.source().is_none());
}
