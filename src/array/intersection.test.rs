// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn keeps_elements_present_in_both_in_a_order() {
    assert_eq!(intersection(&[3, 1, 2], &[2, 3]), [3, 2]);
}

#[test]
fn keeps_duplicates_from_a() {
    assert_eq!(intersection(&[1, 2, 3, 2], &[2, 3, 4]), [2, 3, 2]);
}

#[test]
fn empty_sides_and_disjoint_inputs() {
    assert!(intersection::<i32>(&[], &[1]).is_empty());
    assert!(intersection(&[1], &[]).is_empty());
    assert!(intersection(&[1], &[2]).is_empty());
}
