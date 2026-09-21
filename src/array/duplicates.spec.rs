// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn matches_a_count_based_definition(items in prop::collection::vec(0i32..20, 0..40)) {
        let out = duplicates(&items);
        prop_assert_eq!(out.len(), out.iter().collect::<HashSet<_>>().len());
        for x in &out {
            prop_assert!(items.iter().filter(|i| *i == x).count() > 1);
        }
        for x in &items {
            if items.iter().filter(|i| *i == x).count() > 1 {
                prop_assert!(out.contains(x));
            }
        }
    }
}
