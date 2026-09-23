// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn the_total_granted_never_exceeds_capacity_plus_what_the_time_earned(
        capacity in 1u32..10,
        rate in 0u32..5,
        gaps in prop::collection::vec(0u64..2000, 1..30),
    ) {
        let mut bucket = TokenBucket::new(capacity, rate, 0);
        let (mut now, mut granted) = (0u64, 0u64);
        for gap in gaps {
            now += gap;
            if bucket.try_acquire(now) {
                granted += 1;
            }
        }
        prop_assert!(granted <= u64::from(capacity) + now * u64::from(rate) / 1000);
    }
}
