// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Rng;
use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn returns_min_of_count_and_len_distinct_positions(len in 0usize..40, count in 0usize..60, seed in any::<u64>()) {
        let items: Vec<usize> = (0..len).collect(); // distinct values stand for positions
        let mut picked = sample(&items, count, &mut Rng::seeded(seed));
        prop_assert_eq!(picked.len(), count.min(len));
        picked.sort_unstable();
        picked.dedup();
        prop_assert_eq!(picked.len(), count.min(len));
        prop_assert!(picked.iter().all(|&p| p < len));
    }
}
