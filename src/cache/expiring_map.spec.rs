// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn behaves_like_a_plain_map_that_sweeps_on_write(
        ops in prop::collection::vec((0u8..6, 0u32..6, 0u32..50, 0u64..40, 0u64..40), 0..80),
    ) {
        let mut real = ExpiringMap::<u32, u32, u64>::new();
        // model: key -> (value, expires_at)
        let mut model: std::collections::HashMap<u32, (u32, u64)> = std::collections::HashMap::new();
        for (op, key, value, extra, now) in ops {
            let expires_at = now + extra;
            let live = |entry: &(u32, u64)| now < entry.1;
            match op {
                0 => {
                    model.retain(|_, e| live(e));
                    let expected = model.insert(key, (value, expires_at)).map(|e| e.0);
                    prop_assert_eq!(real.insert(key, value, expires_at, now), expected);
                }
                1 => {
                    model.retain(|_, e| live(e));
                    let expected = match model.entry(key) {
                        std::collections::hash_map::Entry::Occupied(_) => false,
                        std::collections::hash_map::Entry::Vacant(slot) => {
                            slot.insert((value, expires_at));
                            true
                        }
                    };
                    prop_assert_eq!(real.insert_if_absent(key, value, expires_at, now), expected);
                }
                2 => {
                    let expected = model.get(&key).filter(|e| live(e)).map(|e| &e.0);
                    prop_assert_eq!(real.get(&key, now), expected);
                }
                3 => {
                    let expected = model.remove(&key).filter(|e| live(e)).map(|e| e.0);
                    prop_assert_eq!(real.remove(&key, now), expected);
                }
                4 => {
                    let before = model.len();
                    model.retain(|_, e| live(e));
                    prop_assert_eq!(real.evict_expired(now), before - model.len());
                }
                _ => {
                    let expected = model.get(&key).is_some_and(live);
                    prop_assert_eq!(real.contains_key(&key, now), expected);
                }
            }
            prop_assert_eq!(real.len(), model.len());
        }
    }
}
