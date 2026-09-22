// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn finds_the_smallest_and_the_largest() {
    assert_eq!(min_max([3, 1, 4, 1, 5, 9, 2, 6]), Some((1, 9)));
}

#[test]
fn a_single_item_is_both_the_min_and_the_max() {
    assert_eq!(min_max([7]), Some((7, 7)));
}

#[test]
fn empty_input_gives_none() {
    assert_eq!(min_max(Vec::<i32>::new()), None);
}

#[test]
fn works_on_a_non_slice_iterator() {
    assert_eq!(min_max((1..10).filter(|n| n % 3 == 0)), Some((3, 9)));
}

#[test]
fn a_leading_nan_is_never_replaced() {
    let (min, max) = min_max([f64::NAN, 1.0, 2.0]).unwrap();
    assert!(min.is_nan() && max.is_nan());
}

#[test]
fn a_nan_after_the_first_item_is_ignored() {
    assert_eq!(min_max([1.0, f64::NAN, 2.0]), Some((1.0, 2.0)));
}
