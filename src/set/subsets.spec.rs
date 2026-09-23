// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn has_two_to_the_n_distinct_subsets(items in prop::collection::hash_set(0u8..50, 0..8)) {
        let items: Vec<u8> = items.into_iter().collect();
        let all = subsets(&items).unwrap();
        prop_assert_eq!(all.len(), 1usize << items.len());
        let distinct: std::collections::HashSet<_> = all.iter().collect();
        prop_assert_eq!(distinct.len(), all.len());
        prop_assert!(all.iter().all(|s| s.iter().all(|x| items.contains(x))));
    }
}
