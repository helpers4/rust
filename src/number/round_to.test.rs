// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// Exact comparisons are the point: these helpers promise exact results at the boundaries.
#![allow(clippy::float_cmp)]

use super::*;

#[test]
fn rounds_to_the_requested_places() {
    assert_eq!(round_to(1.23456, 2), 1.23);
    assert_eq!(round_to(1.23456, 4), 1.2346);
}

#[test]
fn rounds_half_away_from_zero() {
    assert_eq!(round_to(2.5, 0), 3.0);
    assert_eq!(round_to(-2.5, 0), -3.0);
}

#[test]
fn zero_decimals_rounds_to_an_integer() {
    assert_eq!(round_to(7.4, 0), 7.0);
    assert_eq!(round_to(7.6, 0), 8.0);
}

#[test]
fn integers_are_unchanged() {
    assert_eq!(round_to(1234.0, 3), 1234.0);
}

#[test]
fn non_finite_and_overflowing_values_are_returned_unchanged() {
    assert!(round_to(f64::NAN, 2).is_nan());
    assert_eq!(round_to(f64::INFINITY, 2), f64::INFINITY);
    assert_eq!(round_to(1e308, 5), 1e308);
    assert_eq!(round_to(1.5, u32::MAX), 1.5);
}
