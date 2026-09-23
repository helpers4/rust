// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn merges_every_set() {
    let sets = [
        HashSet::from([1, 2]),
        HashSet::from([2, 3]),
        HashSet::from([4]),
    ];
    assert_eq!(union_all(&sets), HashSet::from([1, 2, 3, 4]));
}

#[test]
fn a_single_set_is_copied() {
    let sets = [HashSet::from([1, 2])];
    assert_eq!(union_all(&sets), HashSet::from([1, 2]));
}

#[test]
fn no_sets_gives_an_empty_set() {
    let sets: [HashSet<i32>; 0] = [];
    assert!(union_all(&sets).is_empty());
}
