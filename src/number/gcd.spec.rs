// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// Exact comparisons are the point: these helpers promise exact results at the boundaries.
#![allow(clippy::float_cmp)]

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn divides_both_and_is_symmetric(a in 1u64..1_000_000, b in 1u64..1_000_000) {
        let g = gcd(a, b);
        prop_assert!(g >= 1 && a % g == 0 && b % g == 0);
        prop_assert_eq!(gcd(b, a), g);
    }

    #[test]
    fn the_quotients_are_coprime(a in 1u64..1_000_000, b in 1u64..1_000_000) {
        let g = gcd(a, b);
        prop_assert_eq!(gcd(a / g, b / g), 1);
    }
}
