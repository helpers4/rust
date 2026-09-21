// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// Exact comparisons are the point: these helpers promise exact results at the boundaries.
#![allow(clippy::float_cmp)]

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_idempotent(v in -1e9f64..1e9, d in 0u32..6) {
        let once = round_to(v, d);
        prop_assert_eq!(round_to(once, d), once);
    }

    #[test]
    fn moves_the_value_by_at_most_half_a_unit_in_the_last_place(v in -1e6f64..1e6, d in 0u32..6) {
        let half_unit = 0.5 / 10f64.powi(i32::try_from(d).unwrap());
        prop_assert!((round_to(v, d) - v).abs() <= half_unit + 1e-9);
    }
}
