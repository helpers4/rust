// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn the_item_is_always_in_the_result_and_the_length_grows_by_at_most_one(items in prop::collection::vec(0i32..10, 0..20), item in 100i32..110, wanted in 0i32..10) {
        let out = replace_or_append(&items, item, |x| *x == wanted);
        prop_assert!(out.contains(&item));
        let expected_len = if items.contains(&wanted) { items.len() } else { items.len() + 1 };
        prop_assert_eq!(out.len(), expected_len);
    }
}
