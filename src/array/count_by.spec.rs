// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn counts_add_up_to_the_input_length(items in prop::collection::vec(0i32..50, 0..40)) {
        let counts = count_by(&items, |x| x % 5);
        prop_assert_eq!(counts.values().sum::<usize>(), items.len());
        prop_assert!(counts.values().all(|&c| c > 0));
    }
}
