// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_sorted_and_keeps_every_element(set in prop::collection::hash_set(0i32..100, 0..20)) {
        let sorted = to_sorted_vec(&set);
        prop_assert_eq!(sorted.len(), set.len());
        prop_assert!(sorted.windows(2).all(|w| w[0] < w[1]));
    }
}
