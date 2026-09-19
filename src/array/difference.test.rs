// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn removes_every_occurrence_of_excluded_values() {
    assert_eq!(difference(&[1, 2, 3, 2], &[2]), [1, 3]);
}

#[test]
fn keeps_duplicates_that_are_not_excluded() {
    assert_eq!(difference(&[1, 1, 2], &[3]), [1, 1, 2]);
}

#[test]
fn empty_sides() {
    assert!(difference::<i32>(&[], &[1]).is_empty());
    assert_eq!(difference(&[1, 2], &[]), [1, 2]);
}
