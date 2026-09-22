// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn matches_separate_min_and_max_calls(items in prop::collection::vec(-1000i32..1000, 1..40)) {
        let (min, max) = min_max(items.clone()).unwrap();
        prop_assert_eq!(Some(min), items.iter().copied().min());
        prop_assert_eq!(Some(max), items.iter().copied().max());
    }

    #[test]
    fn the_min_never_exceeds_the_max(items in prop::collection::vec(-1000i32..1000, 1..40)) {
        let (min, max) = min_max(items).unwrap();
        prop_assert!(min <= max);
    }
}
