// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn returns_the_assigned_value() {
    assert_eq!(get("A=1\nB=2\n", "B").as_deref(), Some("2"));
}

#[test]
fn last_assignment_wins() {
    assert_eq!(get("A=1\nA=2\n", "A").as_deref(), Some("2"));
}

#[test]
fn missing_key_and_empty_content_give_none() {
    assert_eq!(get("A=1\n", "B"), None);
    assert_eq!(get("", "A"), None);
}

#[test]
fn comments_are_not_assignments() {
    assert_eq!(get("# A=1\n", "A"), None);
}

#[test]
fn an_empty_value_is_some_empty() {
    assert_eq!(get("A=\n", "A").as_deref(), Some(""));
}
