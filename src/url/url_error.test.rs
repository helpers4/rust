// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn display_describes_each_variant() {
    assert_eq!(
        UrlError::InvalidCharacter { index: 4 }.to_string(),
        "invalid character at byte 4: percent-encode it"
    );
    assert_eq!(UrlError::MissingScheme.to_string(), "missing scheme");
    assert_eq!(
        UrlError::InvalidScheme { index: 1 }.to_string(),
        "invalid scheme at byte 1"
    );
    assert_eq!(UrlError::InvalidHost.to_string(), "invalid host");
    assert_eq!(UrlError::InvalidPort.to_string(), "invalid port");
}

#[test]
fn is_a_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(UrlError::MissingScheme);
    assert!(err.source().is_none());
}
