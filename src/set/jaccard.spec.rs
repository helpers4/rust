// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_between_zero_and_one_and_symmetric(
        a in prop::collection::hash_set(0u8..20, 0..10),
        b in prop::collection::hash_set(0u8..20, 0..10),
    ) {
        let score = jaccard(&a, &b);
        prop_assert!((0.0..=1.0).contains(&score));
        prop_assert!((score - jaccard(&b, &a)).abs() < f64::EPSILON);
    }
}
