// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// Exact comparisons are the point: these helpers promise exact results at the boundaries.
#![allow(clippy::float_cmp)]

use super::*;

#[test]
fn finds_the_greatest_common_divisor() {
    assert_eq!(gcd(12, 18), 6);
    assert_eq!(gcd(18, 12), 6);
    assert_eq!(gcd(7, 13), 1);
}

#[test]
fn zero_is_neutral() {
    assert_eq!(gcd(0, 5), 5);
    assert_eq!(gcd(5, 0), 5);
    assert_eq!(gcd(0, 0), 0);
}

#[test]
fn handles_the_largest_values() {
    assert_eq!(gcd(u64::MAX, u64::MAX), u64::MAX);
    assert_eq!(gcd(u64::MAX, 1), 1);
}
