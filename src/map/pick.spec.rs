// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_a_submap_restricted_to_the_keys(
        map in prop::collection::hash_map(0u8..20, 0i32..100, 0..15),
        keys in prop::collection::vec(0u8..20, 0..10),
    ) {
        let picked = pick(&map, &keys);
        for (k, v) in &picked {
            prop_assert!(keys.contains(k));
            prop_assert_eq!(map.get(k), Some(v));
        }
        for k in &keys {
            prop_assert_eq!(picked.contains_key(k), map.contains_key(k));
        }
    }
}
