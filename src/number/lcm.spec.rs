// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// Exact comparisons are the point: these helpers promise exact results at the boundaries.
#![allow(clippy::float_cmp)]

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_a_multiple_of_both_and_pairs_with_gcd(a in 1u64..100_000, b in 1u64..100_000) {
        let l = lcm(a, b).unwrap();
        prop_assert!(l % a == 0 && l % b == 0);
        prop_assert_eq!(l * gcd(a, b), a * b);
    }
}
