// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn lists_every_subset_in_bit_order() {
    assert_eq!(
        subsets(&["a", "b", "c"]),
        Some(vec![
            vec![],
            vec!["a"],
            vec!["b"],
            vec!["a", "b"],
            vec!["c"],
            vec!["a", "c"],
            vec!["b", "c"],
            vec!["a", "b", "c"],
        ])
    );
}

#[test]
fn no_items_gives_only_the_empty_subset() {
    assert_eq!(subsets::<i32>(&[]), Some(vec![vec![]]));
}

#[test]
fn accepts_up_to_the_limit() {
    let items = [0u8; MAX_SUBSET_ITEMS];
    assert_eq!(
        subsets(&items).map(|s| s.len()),
        Some(1 << MAX_SUBSET_ITEMS)
    );
}

#[test]
fn refuses_more_than_the_limit() {
    assert_eq!(subsets(&[0u8; MAX_SUBSET_ITEMS + 1]), None);
}
