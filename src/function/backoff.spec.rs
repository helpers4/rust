// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_decreases_and_stays_within_the_bounds(attempt in 1u32..80, base_ms in 1u64..1000) {
        let base = Duration::from_millis(base_ms);
        let max = Duration::from_secs(30);
        let now = backoff(attempt, base, max);
        prop_assert!(now <= max);
        prop_assert!(now >= base.min(max));
        prop_assert!(backoff(attempt + 1, base, max) >= now);
    }
}
