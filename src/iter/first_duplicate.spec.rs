// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn a_none_result_means_every_item_was_unique(items in prop::collection::vec(0i32..30, 0..30)) {
        if first_duplicate(items.clone()).is_none() {
            let unique: std::collections::HashSet<_> = items.iter().collect();
            prop_assert_eq!(unique.len(), items.len());
        }
    }

    #[test]
    fn a_some_result_appears_earlier_a_second_time(items in prop::collection::vec(0i32..10, 1..30)) {
        if let Some(dup) = first_duplicate(items.clone()) {
            prop_assert!(items.iter().filter(|x| **x == dup).count() >= 2);
        }
    }
}
