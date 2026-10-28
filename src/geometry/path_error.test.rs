// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn display_describes_each_variant() {
    assert_eq!(
        ParsePathError::MissingMove.to_string(),
        "path data must start with a move"
    );
    assert_eq!(
        ParsePathError::UnknownCommand { index: 3 }.to_string(),
        "unknown path command at byte 3"
    );
    assert_eq!(
        ParsePathError::ExpectedNumber { index: 4 }.to_string(),
        "expected a number at byte 4"
    );
    assert_eq!(
        ParsePathError::ExpectedFlag { index: 5 }.to_string(),
        "expected an arc flag at byte 5"
    );
}

#[test]
fn is_a_std_error() {
    let error: &dyn std::error::Error = &ParsePathError::MissingMove;
    assert!(error.source().is_none());
}
