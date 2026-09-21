// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// Exact comparisons are the point: these helpers promise exact results at the boundaries.
#![allow(clippy::float_cmp)]

use super::*;

#[test]
fn finds_the_least_common_multiple() {
    assert_eq!(lcm(4, 6), Some(12));
    assert_eq!(lcm(6, 4), Some(12));
    assert_eq!(lcm(5, 5), Some(5));
    assert_eq!(lcm(3, 7), Some(21));
}

#[test]
fn zero_gives_zero() {
    assert_eq!(lcm(0, 5), Some(0));
    assert_eq!(lcm(5, 0), Some(0));
}

#[test]
fn overflow_gives_none() {
    assert_eq!(lcm(u64::MAX, u64::MAX - 1), None);
}

#[test]
fn does_not_overflow_when_the_result_fits() {
    assert_eq!(lcm(u64::MAX, u64::MAX), Some(u64::MAX));
}
