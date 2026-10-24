// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use std::cmp::Ordering::{Equal, Greater, Less};

#[test]
fn compares_numbers_by_value() {
    assert_eq!(compare_natural("frame 2", "frame 10"), Less);
    assert_eq!(compare_natural("frame 10", "frame 2"), Greater);
    assert_eq!(compare_natural("frame 10", "frame 10"), Equal);
    assert_eq!(compare_natural("a9", "a10"), Less);
    assert_eq!(compare_natural("a19", "a2"), Greater);
}

#[test]
fn compares_text_by_code_point() {
    assert_eq!(compare_natural("apple", "banana"), Less);
    assert_eq!(compare_natural("B", "a"), Less);
    assert_eq!(compare_natural("é", "z"), Greater);
}

#[test]
fn a_prefix_comes_first() {
    assert_eq!(compare_natural("", ""), Equal);
    assert_eq!(compare_natural("", "a"), Less);
    assert_eq!(compare_natural("a", ""), Greater);
    assert_eq!(compare_natural("frame", "frame 1"), Less);
    assert_eq!(compare_natural("frame 1", "frame"), Greater);
}

#[test]
fn compares_several_numbers_in_turn() {
    assert_eq!(compare_natural("v1.2.10", "v1.10.2"), Less);
    assert_eq!(compare_natural("v1.2.3", "v1.2.3"), Equal);
    assert_eq!(compare_natural("1a2", "1a10"), Less);
}

#[test]
fn compares_huge_numbers_exactly() {
    let big = "9".repeat(60);
    let bigger = format!("1{}", "0".repeat(60));
    assert_eq!(compare_natural(&big, &bigger), Less);
    assert_eq!(compare_natural(&bigger, &big), Greater);
}

#[test]
fn a_number_against_text_is_compared_as_text() {
    assert_eq!(compare_natural("1", "a"), Less); // '1' < 'a'
    assert_eq!(compare_natural("a", "1"), Greater);
}

#[test]
fn orders_equal_numbers_by_padding_only_when_the_rest_is_equal() {
    assert_eq!(compare_natural("a1", "a01"), Less);
    assert_eq!(compare_natural("a01", "a1"), Greater);
    assert_eq!(compare_natural("a001", "a01"), Greater);
    assert_eq!(compare_natural("a01", "a01"), Equal);
    // A later difference wins over padding.
    assert_eq!(compare_natural("a1b", "a01a"), Greater);
    // The first padding difference decides among several numbers.
    assert_eq!(compare_natural("1-02", "01-2"), Less);
    assert_eq!(compare_natural("0", "00"), Less);
    assert_eq!(compare_natural("0", "0"), Equal);
}

#[test]
fn sorts_a_list_naturally() {
    let mut names = ["w20", "w2", "w11", "W3"];
    names.sort_by(|a, b| compare_natural(a, b));
    assert_eq!(names, ["W3", "w2", "w11", "w20"]);
}
