// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn display_describes_each_variant() {
    assert_eq!(ParseCommitError::Empty.to_string(), "empty commit message");
    assert_eq!(
        ParseCommitError::MissingSeparator.to_string(),
        "expected `type(scope): description`"
    );
    assert_eq!(
        ParseCommitError::InvalidType.to_string(),
        "invalid commit type"
    );
    assert_eq!(
        ParseCommitError::InvalidScope.to_string(),
        "invalid commit scope"
    );
    assert_eq!(
        ParseCommitError::EmptyDescription.to_string(),
        "empty description"
    );
    assert_eq!(
        ParseCommitError::MissingBlankLine.to_string(),
        "the header must be followed by a blank line"
    );
}

#[test]
fn is_a_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(ParseCommitError::Empty);
    assert!(err.source().is_none());
}
