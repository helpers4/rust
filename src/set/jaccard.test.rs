// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// Exact comparisons are the point: these helpers promise exact results at the boundaries.
#![allow(clippy::float_cmp)]

use super::*;

#[test]
fn is_the_intersection_over_the_union() {
    let a = HashSet::from([1, 2, 3]);
    let b = HashSet::from([2, 3, 4]);
    assert_eq!(jaccard(&a, &b), 0.5);
}

#[test]
fn equal_sets_score_one() {
    let a = HashSet::from([1, 2]);
    assert_eq!(jaccard(&a, &a.clone()), 1.0);
}

#[test]
fn disjoint_sets_score_zero() {
    assert_eq!(jaccard(&HashSet::from([1]), &HashSet::from([2])), 0.0);
}

#[test]
fn two_empty_sets_score_one() {
    assert_eq!(jaccard(&HashSet::<i32>::new(), &HashSet::new()), 1.0);
}

#[test]
fn an_empty_and_a_non_empty_set_score_zero() {
    assert_eq!(jaccard(&HashSet::from([1]), &HashSet::new()), 0.0);
}
