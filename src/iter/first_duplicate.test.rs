// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later
use super::*;

#[test]
fn returns_the_first_item_seen_twice() {
    assert_eq!(first_duplicate([3, 1, 4, 1, 5, 9, 4]), Some(1));
}

#[test]
fn no_duplicates_gives_none() {
    assert_eq!(first_duplicate([1, 2, 3]), None);
}

#[test]
fn empty_input_gives_none() {
    assert_eq!(first_duplicate(Vec::<i32>::new()), None);
}

#[test]
fn stops_at_the_first_repeat_and_never_looks_further() {
    // An unbounded iterator: if `first_duplicate` did not stop early, this would hang.
    assert_eq!(first_duplicate((0..).map(|n: u64| n % 3)), Some(0));
}
