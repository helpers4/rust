// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn removes_an_element_that_is_there() {
    assert_eq!(toggle(&["a", "b", "c"], "b"), vec!["a", "c"]);
}

#[test]
fn adds_an_element_that_is_missing_at_the_end() {
    assert_eq!(toggle(&["a", "c"], "b"), vec!["a", "c", "b"]);
    assert_eq!(toggle::<i32>(&[], 1), vec![1]);
}

#[test]
fn removes_only_the_first_occurrence() {
    assert_eq!(toggle(&[1, 2, 1, 3], 1), vec![2, 1, 3]);
}

#[test]
fn leaves_the_input_alone() {
    let items = [1, 2, 3];
    let _ = toggle(&items, 2);
    assert_eq!(items, [1, 2, 3]);
}
