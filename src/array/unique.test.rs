// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn removes_later_duplicates_and_keeps_order() {
    assert_eq!(unique(&[3, 1, 3, 2, 1]), [3, 1, 2]);
}

#[test]
fn empty_and_already_unique_inputs_are_unchanged() {
    assert!(unique::<i32>(&[]).is_empty());
    assert_eq!(unique(&["a", "b"]), ["a", "b"]);
}
