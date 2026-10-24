// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn removes_the_first_element_with_the_same_key() {
    let items = [("a", 1), ("b", 2), ("c", 1)];
    assert_eq!(
        toggle_by(&items, ("z", 1), |x| x.1),
        vec![("b", 2), ("c", 1)]
    );
}

#[test]
fn adds_the_element_when_no_key_matches() {
    assert_eq!(
        toggle_by(&[("a", 1)], ("b", 2), |x| x.1),
        vec![("a", 1), ("b", 2)]
    );
    assert_eq!(
        toggle_by::<(&str, i32), i32>(&[], ("b", 2), |x| x.1),
        vec![("b", 2)]
    );
}

#[test]
fn computes_the_key_of_the_new_element_once() {
    let mut calls = 0;
    let _ = toggle_by(&[1, 2, 3], 2, |x| {
        calls += 1;
        *x
    });
    assert_eq!(calls, 3); // one for the item, one each for 1 and 2 (stops at the match)
}
