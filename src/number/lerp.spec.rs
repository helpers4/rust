// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// Exact comparisons are the point: these helpers promise exact results at the boundaries.
#![allow(clippy::float_cmp)]

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_exact_at_the_ends(a in -1e6f64..1e6, b in -1e6f64..1e6) {
        prop_assert_eq!(lerp(a, b, 0.0), a);
        prop_assert_eq!(lerp(a, b, 1.0), b);
    }

    #[test]
    fn stays_between_the_ends_for_t_in_the_unit_interval(
        a in -1e6f64..1e6,
        b in -1e6f64..1e6,
        t in 0.0f64..=1.0,
    ) {
        let v = lerp(a, b, t);
        prop_assert!(v >= a.min(b) - 1e-6 && v <= a.max(b) + 1e-6);
    }
}
