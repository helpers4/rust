// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn parses_a_positive_value() {
    assert_eq!(get_int("PORT=8080", "PORT", 80), 8080);
}

#[test]
fn parses_a_negative_value() {
    assert_eq!(get_int("OFFSET=-3", "OFFSET", 0), -3);
}

#[test]
fn ignores_surrounding_whitespace() {
    assert_eq!(get_int("PORT=  8080  ", "PORT", 80), 8080);
}

#[test]
fn missing_key_returns_the_default() {
    assert_eq!(get_int("", "PORT", 80), 80);
}

#[test]
fn a_value_that_does_not_parse_returns_the_default() {
    assert_eq!(get_int("PORT=not-a-number", "PORT", 80), 80);
}

#[test]
fn a_value_too_large_for_i64_returns_the_default() {
    assert_eq!(get_int("PORT=99999999999999999999", "PORT", 80), 80);
}
