// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn accepts_ordinary_addresses() {
    assert!(is_valid_email("jane.doe@example.com"));
    assert!(is_valid_email("jane.doe+list@example.co.uk"));
    assert!(is_valid_email("a@b.co"));
}

#[test]
fn rejects_missing_or_extra_at_signs() {
    assert!(!is_valid_email("no-at-sign"));
    assert!(!is_valid_email("a@b@example.com"));
}

#[test]
fn rejects_an_empty_local_part_or_domain() {
    assert!(!is_valid_email("@example.com"));
    assert!(!is_valid_email("a@"));
}

#[test]
fn rejects_a_leading_trailing_or_doubled_dot_in_the_local_part() {
    assert!(!is_valid_email(".a@example.com"));
    assert!(!is_valid_email("a.@example.com"));
    assert!(!is_valid_email("a..b@example.com"));
}

#[test]
fn rejects_a_domain_with_a_single_label() {
    assert!(!is_valid_email("jane@localhost"));
}

#[test]
fn rejects_a_domain_label_that_starts_or_ends_with_a_hyphen() {
    assert!(!is_valid_email("a@-example.com"));
    assert!(!is_valid_email("a@example-.com"));
}

#[test]
fn accepts_a_domain_label_with_an_internal_hyphen() {
    assert!(is_valid_email("a@my-site.example.com"));
}

#[test]
fn rejects_a_numeric_or_single_letter_top_level_label() {
    assert!(!is_valid_email("a@example.123"));
    assert!(!is_valid_email("a@example.c"));
}

#[test]
fn rejects_a_character_that_is_not_allowed_in_the_local_part() {
    assert!(!is_valid_email("a b@example.com"));
    assert!(!is_valid_email("a/b@example.com"));
}
