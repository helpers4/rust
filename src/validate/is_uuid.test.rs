// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn accepts_a_canonical_uuid() {
    assert!(is_uuid("550e8400-e29b-41d4-a716-446655440000"));
}

#[test]
fn is_case_insensitive() {
    assert!(is_uuid("550E8400-E29B-41D4-A716-446655440000"));
}

#[test]
fn accepts_the_nil_uuid() {
    assert!(is_uuid("00000000-0000-0000-0000-000000000000"));
}

#[test]
fn rejects_the_wrong_length() {
    assert!(!is_uuid("550e8400-e29b-41d4-a716-44665544000"));
    assert!(!is_uuid("550e8400-e29b-41d4-a716-4466554400000"));
    assert!(!is_uuid(""));
}

#[test]
fn rejects_hyphens_in_the_wrong_place() {
    assert!(!is_uuid("550e8400ee29b-41d4-a716-446655440000"));
}

#[test]
fn rejects_a_non_hex_character() {
    assert!(!is_uuid("550e8400-e29b-41d4-a716-44665544000g"));
}
