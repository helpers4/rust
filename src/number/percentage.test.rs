// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// Exact comparisons are the point: these helpers promise exact results at the boundaries.
#![allow(clippy::float_cmp)]

use super::*;

#[test]
fn computes_the_share() {
    assert_eq!(percentage(25.0, 200.0), Some(12.5));
    assert_eq!(percentage(50.0, 50.0), Some(100.0));
    assert_eq!(percentage(0.0, 10.0), Some(0.0));
}

#[test]
fn is_not_clamped() {
    assert_eq!(percentage(3.0, 2.0), Some(150.0));
    assert_eq!(percentage(-1.0, 4.0), Some(-25.0));
}

#[test]
fn a_zero_total_gives_none() {
    assert_eq!(percentage(1.0, 0.0), None);
    assert_eq!(percentage(1.0, -0.0), None);
}
