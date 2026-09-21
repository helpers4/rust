// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// Exact comparisons are the point: these helpers promise exact results at the boundaries.
#![allow(clippy::float_cmp)]

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn does_not_depend_on_the_order(
        mut values in prop::collection::vec(-1e6f64..1e6, 1..30),
    ) {
        let forward = median(&values);
        values.reverse();
        prop_assert_eq!(median(&values), forward);
    }

    #[test]
    fn lies_between_the_smallest_and_the_largest(
        values in prop::collection::vec(-1e6f64..1e6, 1..30),
    ) {
        let m = median(&values).unwrap();
        let min = values.iter().copied().fold(f64::INFINITY, f64::min);
        let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        prop_assert!(m >= min && m <= max);
    }
}
