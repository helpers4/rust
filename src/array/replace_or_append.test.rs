// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn replaces_the_first_match_in_place() {
    assert_eq!(
        replace_or_append(&[1, 2, 3, 2], 9, |x| *x == 2),
        vec![1, 9, 3, 2]
    );
}

#[test]
fn appends_when_nothing_matches() {
    assert_eq!(replace_or_append(&[1, 2], 9, |x| *x == 5), vec![1, 2, 9]);
    assert_eq!(replace_or_append::<i32>(&[], 9, |_| true), vec![9]);
}

#[test]
fn leaves_the_input_alone() {
    let items = [1, 2, 3];
    let _ = replace_or_append(&items, 9, |x| *x == 2);
    assert_eq!(items, [1, 2, 3]);
}
