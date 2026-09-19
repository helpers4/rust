// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn counts_per_key() {
    let counts = count_by(&[1, 2, 3, 4, 5], |n| n % 2 == 0);
    assert_eq!(counts[&false], 3);
    assert_eq!(counts[&true], 2);
    assert_eq!(counts.len(), 2);
}

#[test]
fn empty_input_gives_empty_map() {
    assert!(count_by(&[] as &[i32], |x| *x).is_empty());
}
