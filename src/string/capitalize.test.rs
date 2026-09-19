// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn empty_string_stays_empty() {
    assert_eq!(capitalize(""), "");
}

#[test]
fn uppercases_first_ascii_letter_only() {
    assert_eq!(capitalize("hello World"), "Hello World");
}

#[test]
fn expands_multi_char_uppercase() {
    assert_eq!(capitalize("ßa"), "SSa");
}

#[test]
fn leaves_non_letters_alone() {
    assert_eq!(capitalize("1abc"), "1abc");
}
