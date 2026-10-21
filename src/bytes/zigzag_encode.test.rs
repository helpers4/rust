// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn interleaves_negative_and_positive_values() {
    let encoded: Vec<u64> = [0, -1, 1, -2, 2].into_iter().map(zigzag_encode).collect();
    assert_eq!(encoded, [0, 1, 2, 3, 4]);
}

#[test]
fn encodes_the_extremes() {
    assert_eq!(zigzag_encode(i64::MAX), u64::MAX - 1);
    assert_eq!(zigzag_encode(i64::MIN), u64::MAX);
}

#[test]
fn matches_32_bit_zigzag_for_widened_i32() {
    assert_eq!(zigzag_encode(i64::from(i32::MIN)), u64::from(u32::MAX));
    assert_eq!(zigzag_encode(i64::from(i32::MAX)), u64::from(u32::MAX - 1));
}
