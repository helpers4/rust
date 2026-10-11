// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn pairs_in_row_major_order() {
    assert_eq!(
        cartesian_product(&[1, 2], &['a', 'b']),
        [(1, 'a'), (1, 'b'), (2, 'a'), (2, 'b')]
    );
}

#[test]
fn an_empty_side_gives_no_pairs() {
    assert_eq!(cartesian_product::<i32, i32>(&[], &[1]), []);
    assert_eq!(cartesian_product(&[1], &[] as &[i32]), []);
}
