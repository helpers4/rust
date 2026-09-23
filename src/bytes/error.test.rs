// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn display_describes_each_variant() {
    assert_eq!(ParseSizeError::Empty.to_string(), "empty size");
    assert_eq!(
        ParseSizeError::ExpectedNumber { index: 2 }.to_string(),
        "expected a number at byte 2"
    );
    assert_eq!(
        ParseSizeError::UnknownUnit { index: 3 }.to_string(),
        "unknown unit at byte 3"
    );
    assert_eq!(ParseSizeError::Overflow.to_string(), "size is too large");
}

#[test]
fn is_a_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(ParseSizeError::Empty);
    assert!(err.source().is_none());
}
