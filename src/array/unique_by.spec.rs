// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn output_keys_are_distinct_and_cover_all_input_keys(items in prop::collection::vec(0i32..50, 0..30)) {
        let out = unique_by(&items, |x| x % 3);
        let keys: Vec<_> = out.iter().map(|x| x % 3).collect();
        let distinct: std::collections::HashSet<_> = keys.iter().collect();
        prop_assert_eq!(distinct.len(), keys.len());
        let input_keys: std::collections::HashSet<_> = items.iter().map(|x| x % 3).collect();
        prop_assert_eq!(distinct.len(), input_keys.len());
    }
}
