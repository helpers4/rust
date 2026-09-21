// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn every_key_maps_to_its_last_element(items in prop::collection::vec(0i32..50, 0..40)) {
        let indexed = key_by(&items, |x| x % 5);
        for (k, v) in &indexed {
            prop_assert_eq!(v % 5, *k);
            prop_assert_eq!(items.iter().rev().find(|x| *x % 5 == *k), Some(v));
        }
        prop_assert!(indexed.len() <= 5);
    }
}
