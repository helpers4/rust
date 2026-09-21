// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// Exact comparisons are the point: these helpers promise exact results at the boundaries.
#![allow(clippy::float_cmp)]

use super::*;

#[test]
fn odd_count_gives_the_middle_value() {
    assert_eq!(median(&[3.0, 1.0, 2.0]), Some(2.0));
    assert_eq!(median(&[9.0]), Some(9.0));
}

#[test]
fn even_count_gives_the_midpoint_of_the_middle_two() {
    assert_eq!(median(&[4.0, 1.0, 3.0, 2.0]), Some(2.5));
    assert_eq!(median(&[1.0, 3.0]), Some(2.0));
}

#[test]
fn does_not_overflow_on_large_values() {
    assert_eq!(median(&[f64::MAX, f64::MAX]), Some(f64::MAX));
}

#[test]
fn empty_or_nan_gives_none() {
    assert_eq!(median(&[]), None);
    assert_eq!(median(&[1.0, f64::NAN, 2.0]), None);
}

#[test]
fn does_not_modify_the_input() {
    let values = [3.0, 1.0, 2.0];
    let _ = median(&values);
    assert_eq!(values, [3.0, 1.0, 2.0]);
}
