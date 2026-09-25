// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_exceeds_capacity(capacity in 0usize..8, keys in prop::collection::vec(0u8..16, 0..50)) {
        let mut cache: LruCache<u8, u8> = LruCache::new(capacity);
        for key in keys {
            cache.insert(key, key);
            prop_assert!(cache.len() <= capacity);
        }
    }

    #[test]
    fn a_value_just_inserted_is_retrievable_when_capacity_allows_it(
        capacity in 1usize..8, key in 0u8..16, value in 0i32..1000,
    ) {
        let mut cache: LruCache<u8, i32> = LruCache::new(capacity);
        cache.insert(key, value);
        prop_assert_eq!(cache.get(&key), Some(&value));
    }

    #[test]
    fn get_or_insert_with_is_idempotent_for_a_repeated_key(
        capacity in 1usize..8, key in 0u8..16, value in 0i32..1000,
    ) {
        let mut cache: LruCache<u8, i32> = LruCache::new(capacity);
        let first = cache.get_or_insert_with(key, || value);
        let second = cache.get_or_insert_with(key, || value);
        prop_assert_eq!(first, second);
    }
}
