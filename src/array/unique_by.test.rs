// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn keeps_the_first_element_of_each_key() {
    let words = ["apple", "avocado", "banana", "blueberry", "cherry"];
    assert_eq!(
        unique_by(&words, |w| w.chars().next()),
        ["apple", "banana", "cherry"]
    );
}

#[test]
fn empty_input_gives_empty_output() {
    assert!(unique_by(&[] as &[i32], |x| *x).is_empty());
}

#[test]
fn key_function_is_called_once_per_element() {
    let mut calls = 0;
    unique_by(&[1, 2, 3, 4], |x| {
        calls += 1;
        x % 2
    });
    assert_eq!(calls, 4);
}
