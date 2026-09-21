// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn display_describes_each_variant() {
    assert_eq!(ParseDurationError::Empty.to_string(), "empty duration");
    assert_eq!(
        ParseDurationError::ExpectedNumber { index: 2 }.to_string(),
        "expected a number at byte 2"
    );
    assert_eq!(
        ParseDurationError::MissingUnit { index: 3 }.to_string(),
        "expected a unit at byte 3"
    );
    assert_eq!(
        ParseDurationError::UnknownUnit { index: 1 }.to_string(),
        "unknown unit at byte 1"
    );
    assert_eq!(
        ParseDurationError::Overflow.to_string(),
        "duration is too large"
    );
}

#[test]
fn is_a_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(ParseDurationError::Empty);
    assert!(err.source().is_none());
}
