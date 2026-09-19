// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn groups_and_keeps_order_inside_groups() {
    let groups = group_by(&[1, 2, 3, 4, 5], |n| n % 2 == 0);
    assert_eq!(groups[&true], [2, 4]);
    assert_eq!(groups[&false], [1, 3, 5]);
    assert_eq!(groups.len(), 2);
}

#[test]
fn empty_input_gives_empty_map() {
    assert!(group_by(&[] as &[i32], |x| *x).is_empty());
}
