// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn pair(key: &str, value: &str) -> (String, String) {
    (key.to_string(), value.to_string())
}

#[test]
fn parses_pairs_in_order() {
    assert_eq!(
        parse_query("a=1&b=2").unwrap(),
        vec![pair("a", "1"), pair("b", "2")]
    );
}

#[test]
fn ignores_a_leading_question_mark() {
    assert_eq!(parse_query("?a=1").unwrap(), vec![pair("a", "1")]);
}

#[test]
fn decodes_plus_and_percent_escapes() {
    assert_eq!(
        parse_query("q=rust+lang&x=%C3%A9&k%20ey=v").unwrap(),
        vec![pair("q", "rust lang"), pair("x", "é"), pair("k ey", "v")]
    );
}

#[test]
fn keeps_repeated_keys() {
    assert_eq!(
        parse_query("t=a&t=b").unwrap(),
        vec![pair("t", "a"), pair("t", "b")]
    );
}

#[test]
fn a_pair_without_equals_has_an_empty_value() {
    assert_eq!(
        parse_query("flag&x=1").unwrap(),
        vec![pair("flag", ""), pair("x", "1")]
    );
}

#[test]
fn only_the_first_equals_splits() {
    assert_eq!(parse_query("a=b=c").unwrap(), vec![pair("a", "b=c")]);
}

#[test]
fn skips_empty_pairs_and_an_empty_query() {
    assert_eq!(
        parse_query("a=1&&b=2&").unwrap(),
        vec![pair("a", "1"), pair("b", "2")]
    );
    assert!(parse_query("").unwrap().is_empty());
    assert!(parse_query("?").unwrap().is_empty());
}

#[test]
fn an_invalid_escape_in_a_key_or_a_value_is_an_error() {
    assert_eq!(
        parse_query("a%=1"),
        Err(PercentDecodeError::InvalidEscape { index: 1 })
    );
    assert_eq!(
        parse_query("a=%zz"),
        Err(PercentDecodeError::InvalidEscape { index: 0 })
    );
}
