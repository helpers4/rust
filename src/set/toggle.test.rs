// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn inserts_a_missing_element() {
    let mut set = HashSet::from([1]);
    assert!(toggle(&mut set, 2));
    assert_eq!(set, HashSet::from([1, 2]));
}

#[test]
fn removes_a_present_element() {
    let mut set = HashSet::from([1, 2]);
    assert!(!toggle(&mut set, 1));
    assert_eq!(set, HashSet::from([2]));
}
