// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn equal_regardless_of_order() {
    assert!(equals_unordered(&[1, 2, 2, 3], &[3, 2, 1, 2]));
    assert!(equals_unordered::<i32>(&[], &[]));
}

#[test]
fn different_lengths_are_not_equal() {
    assert!(!equals_unordered(&[1, 2], &[1, 2, 3]));
}

#[test]
fn different_multiplicities_are_not_equal() {
    assert!(!equals_unordered(&[1, 2, 2], &[1, 1, 2]));
}

#[test]
fn a_value_missing_from_the_other_side_is_not_equal() {
    assert!(!equals_unordered(&[1, 2], &[1, 3]));
}
