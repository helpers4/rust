// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn display_describes_each_variant() {
    assert_eq!(ParseColorError::Empty.to_string(), "empty color");
    assert_eq!(
        ParseColorError::InvalidLength { length: 4 }.to_string(),
        "a hex color has 3 or 6 digits, found 4"
    );
    assert_eq!(
        ParseColorError::InvalidDigit { index: 2 }.to_string(),
        "invalid hex digit at byte 2"
    );
    assert_eq!(
        ParseColorError::InvalidFunction.to_string(),
        "rgb() takes three whole numbers"
    );
    assert_eq!(
        ParseColorError::OutOfRange.to_string(),
        "a channel is above 255"
    );
    assert_eq!(ParseColorError::UnknownName.to_string(), "unknown color");
}

#[test]
fn is_a_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(ParseColorError::Empty);
    assert!(err.source().is_none());
}
