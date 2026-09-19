// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn replaces_in_place_and_keeps_other_lines() {
    let out = set("# c\nA=1\nB=2\n", "A", "9").unwrap();
    assert_eq!(out, "# c\nA=9\nB=2\n");
}

#[test]
fn appends_when_missing() {
    assert_eq!(set("A=1\n", "B", "2").unwrap(), "A=1\nB=2\n");
    assert_eq!(set("", "B", "2").unwrap(), "B=2\n");
}

#[test]
fn appends_after_a_last_line_without_newline() {
    assert_eq!(set("A=1", "B", "2").unwrap(), "A=1\nB=2\n");
}

#[test]
fn drops_later_duplicates_of_the_key() {
    assert_eq!(set("A=1\nB=2\nA=3\n", "A", "9").unwrap(), "A=9\nB=2\n");
}

#[test]
fn keeps_the_line_ending_of_the_replaced_line() {
    assert_eq!(set("A=1\r\nB=2\r\n", "A", "9").unwrap(), "A=9\r\nB=2\r\n");
    assert_eq!(set("A=1", "A", "9").unwrap(), "A=9");
}

#[test]
fn replaces_exported_and_spaced_assignments() {
    assert_eq!(set("export A = 1 # note\n", "A", "9").unwrap(), "A=9\n");
}

#[test]
fn does_not_touch_comments_that_mention_the_key() {
    assert_eq!(set("# A=1\n", "A", "2").unwrap(), "# A=1\nA=2\n");
}

#[test]
fn quotes_values_that_need_it() {
    assert_eq!(set("", "A", "two words").unwrap(), "A=\"two words\"\n");
}

#[test]
fn rejects_invalid_keys() {
    let err = set("", "not valid", "x").unwrap_err();
    assert_eq!(err.key(), "not valid");
    assert!(set("", "", "x").is_err());
}
