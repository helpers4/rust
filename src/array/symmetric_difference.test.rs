// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn returns_a_only_then_b_only() {
    assert_eq!(symmetric_difference(&[1, 2, 3], &[2, 3, 4]), [1, 4]);
}

#[test]
fn keeps_duplicates() {
    assert_eq!(symmetric_difference(&[1, 1, 2], &[2, 3, 3]), [1, 1, 3, 3]);
}

#[test]
fn identical_inputs_cancel_out_and_empty_side_returns_the_other() {
    assert!(symmetric_difference(&[1, 2], &[2, 1]).is_empty());
    assert_eq!(symmetric_difference(&[], &[1, 2]), [1, 2]);
}
