// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn keeps_every_element_under_its_own_key(items in prop::collection::vec(0i32..50, 0..40)) {
        let groups = group_by(&items, |x| x % 5);
        prop_assert_eq!(groups.values().map(Vec::len).sum::<usize>(), items.len());
        for (k, members) in &groups {
            prop_assert!(members.iter().all(|x| x % 5 == *k));
            let expected: Vec<i32> = items.iter().filter(|x| *x % 5 == *k).copied().collect();
            prop_assert_eq!(members, &expected);
        }
    }
}
