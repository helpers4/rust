// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn toggling_twice_gives_back_the_original_for_distinct_elements(set in prop::collection::hash_set(0i32..100, 0..20), item in 0i32..100) {
        let items: Vec<i32> = set.into_iter().collect();
        let twice = toggle(&toggle(&items, item), item);
        let mut a = items.clone();
        let mut b = twice;
        a.sort_unstable();
        b.sort_unstable();
        prop_assert_eq!(a, b);
    }

    #[test]
    fn the_length_changes_by_exactly_one(items in prop::collection::vec(0i32..10, 0..20), item in 0i32..10) {
        let toggled = toggle(&items, item);
        prop_assert_eq!(toggled.len().abs_diff(items.len()), 1);
        prop_assert_eq!(toggled.contains(&item), !items.contains(&item) || items.iter().filter(|&&x| x == item).count() > 1);
    }
}
