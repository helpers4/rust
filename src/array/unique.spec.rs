// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_idempotent_and_has_no_duplicates(items in prop::collection::vec(0i8..8, 0..30)) {
        let once = unique(&items);
        prop_assert_eq!(unique(&once), once.clone());
        let distinct: std::collections::HashSet<_> = once.iter().collect();
        prop_assert_eq!(distinct.len(), once.len());
    }

    #[test]
    fn only_contains_input_values(items in prop::collection::vec(0i8..8, 0..30)) {
        prop_assert!(unique(&items).iter().all(|x| items.contains(x)));
    }
}
