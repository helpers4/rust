// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn identical_strings_have_zero_distance() {
    assert_eq!(levenshtein_distance("same", "same", true), 0);
}

#[test]
fn classic_example() {
    assert_eq!(levenshtein_distance("kitten", "sitting", true), 3);
}

#[test]
fn empty_first_string_is_the_length_of_the_second() {
    assert_eq!(levenshtein_distance("", "abc", true), 3);
}

#[test]
fn empty_second_string_is_the_length_of_the_first() {
    assert_eq!(levenshtein_distance("abc", "", true), 3);
}

#[test]
fn both_empty_is_zero() {
    assert_eq!(levenshtein_distance("", "", true), 0);
}

#[test]
fn case_sensitive_by_default_behavior_differs_from_case_insensitive() {
    assert_eq!(levenshtein_distance("Kitten", "kitten", true), 1);
    assert_eq!(levenshtein_distance("Kitten", "kitten", false), 0);
}

#[test]
fn counts_unicode_scalar_values_not_bytes() {
    assert_eq!(levenshtein_distance("café", "cafe", true), 1);
}
