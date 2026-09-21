// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// Exact comparisons are the point: these helpers promise exact results at the boundaries.
#![allow(clippy::float_cmp)]

use super::*;

#[test]
fn averages_the_values() {
    assert_eq!(mean(&[1.0, 2.0, 6.0]), Some(3.0));
    assert_eq!(mean(&[-2.0, 2.0]), Some(0.0));
}

#[test]
fn single_value_is_its_own_mean() {
    assert_eq!(mean(&[4.5]), Some(4.5));
}

#[test]
fn empty_gives_none() {
    assert_eq!(mean(&[]), None);
}

#[test]
fn nan_propagates() {
    assert!(mean(&[1.0, f64::NAN]).is_some_and(f64::is_nan));
}
