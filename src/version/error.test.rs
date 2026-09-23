// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn display_describes_each_variant() {
    assert_eq!(ParseVersionError::Empty.to_string(), "empty version");
    assert_eq!(
        ParseVersionError::MissingComponent { component: "patch" }.to_string(),
        "missing patch version"
    );
    assert_eq!(
        ParseVersionError::InvalidNumber { component: "minor" }.to_string(),
        "invalid minor version number"
    );
    assert_eq!(
        ParseVersionError::LeadingZero { component: "major" }.to_string(),
        "leading zero in the major version"
    );
    assert_eq!(
        ParseVersionError::InvalidIdentifier.to_string(),
        "invalid pre-release or build identifier"
    );
    assert_eq!(
        ParseVersionError::InvalidComparator.to_string(),
        "invalid version requirement"
    );
}

#[test]
fn is_a_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(ParseVersionError::Empty);
    assert!(err.source().is_none());
}
