// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn keeps_the_keys(map in prop::collection::hash_map(0u8..20, 0i32..100, 0..15)) {
        let out = map_values(&map, |v| i64::from(*v) + 1);
        prop_assert_eq!(out.len(), map.len());
        for (k, v) in &map {
            prop_assert_eq!(out.get(k), Some(&(i64::from(*v) + 1)));
        }
    }
}
