// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn alternates_equal_lengths() {
    assert_eq!(interleave(&[1, 3], &[2, 4]), vec![1, 2, 3, 4]);
}

#[test]
fn appends_the_leftover_of_the_first() {
    assert_eq!(interleave(&[1, 3, 5, 7], &[2]), vec![1, 2, 3, 5, 7]);
}

#[test]
fn appends_the_leftover_of_the_second() {
    assert_eq!(interleave(&[1], &[2, 4, 6]), vec![1, 2, 4, 6]);
}

#[test]
fn empty_sides() {
    assert_eq!(interleave(&[] as &[i32], &[1, 2]), vec![1, 2]);
    assert_eq!(interleave(&[1, 2], &[]), vec![1, 2]);
    assert_eq!(interleave(&[] as &[i32], &[]), Vec::<i32>::new());
}
