// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn display_describes_each_variant() {
    assert_eq!(HostnameError::Empty.to_string(), "hostname is empty");
    assert_eq!(
        HostnameError::TooLong.to_string(),
        "hostname is longer than 253 octets"
    );
    assert_eq!(
        HostnameError::EmptyLabel.to_string(),
        "hostname has an empty label"
    );
    assert_eq!(
        HostnameError::LabelTooLong.to_string(),
        "hostname has a label longer than 63 octets"
    );
    assert_eq!(
        HostnameError::InvalidChar {
            index: 4,
            found: '_'
        }
        .to_string(),
        "invalid hostname character '_' at byte 4"
    );
    assert_eq!(
        HostnameError::HyphenEdge.to_string(),
        "hostname label starts or ends with a hyphen"
    );
}

#[test]
fn is_a_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(HostnameError::Empty);
    assert!(err.source().is_none());
}
