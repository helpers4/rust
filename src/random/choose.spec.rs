// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Rng;
use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn returns_an_element_of_the_slice(items in prop::collection::vec(any::<i32>(), 1..40), seed in any::<u64>()) {
        let picked = choose(&items, &mut Rng::seeded(seed));
        prop_assert!(picked.is_some_and(|p| items.contains(p)));
    }

    #[test]
    fn is_none_only_for_an_empty_slice(items in prop::collection::vec(any::<u8>(), 0..5), seed in any::<u64>()) {
        prop_assert_eq!(choose(&items, &mut Rng::seeded(seed)).is_none(), items.is_empty());
    }
}
