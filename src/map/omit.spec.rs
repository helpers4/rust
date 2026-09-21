// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn together_with_pick_it_splits_the_map(
        map in prop::collection::hash_map(0u8..20, 0i32..100, 0..15),
        keys in prop::collection::vec(0u8..20, 0..10),
    ) {
        let picked = crate::map::pick(&map, &keys);
        let omitted = omit(&map, &keys);
        prop_assert_eq!(picked.len() + omitted.len(), map.len());
        prop_assert!(picked.keys().all(|k| !omitted.contains_key(k)));
    }
}
