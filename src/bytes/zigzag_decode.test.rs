// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn deinterleaves_values() {
    let decoded: Vec<i64> = [0, 1, 2, 3, 4].into_iter().map(zigzag_decode).collect();
    assert_eq!(decoded, [0, -1, 1, -2, 2]);
}

#[test]
fn decodes_the_extremes() {
    assert_eq!(zigzag_decode(u64::MAX - 1), i64::MAX);
    assert_eq!(zigzag_decode(u64::MAX), i64::MIN);
}
