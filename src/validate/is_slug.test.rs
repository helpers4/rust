// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn accepts_a_well_formed_slug() {
    assert!(is_slug("hello-world"));
    assert!(is_slug("a"));
    assert!(is_slug("v2-beta"));
}

#[test]
fn rejects_uppercase() {
    assert!(!is_slug("Hello-World"));
}

#[test]
fn rejects_leading_or_trailing_hyphens() {
    assert!(!is_slug("-hello"));
    assert!(!is_slug("hello-"));
}

#[test]
fn rejects_a_doubled_hyphen() {
    assert!(!is_slug("hello--world"));
}

#[test]
fn rejects_anything_that_is_not_ascii_alphanumeric_or_a_hyphen() {
    assert!(!is_slug("hello_world"));
    assert!(!is_slug("hello world"));
    assert!(!is_slug("café"));
}

#[test]
fn rejects_empty() {
    assert!(!is_slug(""));
}
