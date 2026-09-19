// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn true_when_an_element_is_shared() {
    assert!(intersects(&[1, 2, 3], &[3, 4]));
}

#[test]
fn false_when_disjoint_or_either_side_is_empty() {
    assert!(!intersects(&[1, 2], &[3, 4]));
    assert!(!intersects::<i32>(&[], &[1]));
    assert!(!intersects(&[1], &[]));
}
