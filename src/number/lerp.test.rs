// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// Exact comparisons are the point: these helpers promise exact results at the boundaries.
#![allow(clippy::float_cmp)]

use super::*;

#[test]
fn hits_both_ends() {
    assert_eq!(lerp(3.0, 9.0, 0.0), 3.0);
    assert_eq!(lerp(3.0, 9.0, 1.0), 9.0);
}

#[test]
fn interpolates_between() {
    assert_eq!(lerp(10.0, 20.0, 0.5), 15.0);
    assert_eq!(lerp(-10.0, 10.0, 0.25), -5.0);
}

#[test]
fn extrapolates_outside_the_unit_interval() {
    assert_eq!(lerp(0.0, 10.0, 1.5), 15.0);
    assert_eq!(lerp(0.0, 10.0, -0.5), -5.0);
}
