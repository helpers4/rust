// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn display_describes_each_variant() {
    assert_eq!(
        PercentDecodeError::InvalidEscape { index: 3 }.to_string(),
        "invalid percent escape at byte 3"
    );
    assert_eq!(
        PercentDecodeError::InvalidUtf8.to_string(),
        "decoded bytes are not valid UTF-8"
    );
}

#[test]
fn is_a_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(PercentDecodeError::InvalidUtf8);
    assert!(err.source().is_none());
}
