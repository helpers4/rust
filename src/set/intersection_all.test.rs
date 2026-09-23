// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn keeps_only_the_common_elements() {
    let sets = [
        HashSet::from([1, 2, 3]),
        HashSet::from([2, 3, 4]),
        HashSet::from([3, 5]),
    ];
    assert_eq!(intersection_all(&sets), HashSet::from([3]));
}

#[test]
fn a_single_set_is_copied() {
    let sets = [HashSet::from([1, 2])];
    assert_eq!(intersection_all(&sets), HashSet::from([1, 2]));
}

#[test]
fn disjoint_sets_give_an_empty_set() {
    let sets = [HashSet::from([1]), HashSet::from([2])];
    assert!(intersection_all(&sets).is_empty());
}

#[test]
fn no_sets_gives_an_empty_set() {
    let sets: [HashSet<i32>; 0] = [];
    assert!(intersection_all(&sets).is_empty());
}
