// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Rng;
use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_a_permutation(items in prop::collection::vec(any::<i32>(), 0..40), seed in any::<u64>()) {
        let mut out = shuffle(&items, &mut Rng::seeded(seed));
        let mut expected = items.clone();
        out.sort_unstable();
        expected.sort_unstable();
        prop_assert_eq!(out, expected);
    }

    #[test]
    fn depends_only_on_the_seed(items in prop::collection::vec(any::<u8>(), 0..40), seed in any::<u64>()) {
        prop_assert_eq!(
            shuffle(&items, &mut Rng::seeded(seed)),
            shuffle(&items, &mut Rng::seeded(seed))
        );
    }
}
