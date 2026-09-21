// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn capitalizes_each_word() {
    assert_eq!(title_case("the quick brown fox"), "The Quick Brown Fox");
}

#[test]
fn lowercases_the_rest_of_each_word() {
    assert_eq!(title_case("HELLO wORLD"), "Hello World");
}

#[test]
fn keeps_whitespace_as_is() {
    assert_eq!(title_case("  a \t b\n"), "  A \t B\n");
}

#[test]
fn only_whitespace_starts_a_word() {
    assert_eq!(title_case("it's well-known"), "It's Well-known");
}

#[test]
fn handles_non_ascii_letters() {
    assert_eq!(title_case("élan vital"), "Élan Vital");
}

#[test]
fn empty_stays_empty() {
    assert_eq!(title_case(""), "");
}
