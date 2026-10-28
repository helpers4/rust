// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn display_describes_each_variant() {
    assert_eq!(
        KiwiError::UnexpectedEof { index: 3 }.to_string(),
        "unexpected end of data at byte 3"
    );
    assert_eq!(
        KiwiError::InvalidString { index: 4 }.to_string(),
        "invalid string at byte 4"
    );
    assert_eq!(
        KiwiError::UnknownKind { kind: 9 }.to_string(),
        "unknown definition kind 9"
    );
    assert_eq!(
        KiwiError::UnknownType { type_id: -9 }.to_string(),
        "unknown type -9"
    );
    assert_eq!(
        KiwiError::UnknownDefinition { name: "X".into() }.to_string(),
        "no definition named \"X\""
    );
    assert_eq!(
        KiwiError::UnknownField {
            message: "M".into(),
            id: 7
        }
        .to_string(),
        "field id 7 does not exist in message \"M\""
    );
    assert_eq!(KiwiError::TooDeep.to_string(), "data nested too deeply");
}

#[test]
fn is_a_std_error() {
    let error: &dyn std::error::Error = &KiwiError::TooDeep;
    assert!(error.source().is_none());
}
