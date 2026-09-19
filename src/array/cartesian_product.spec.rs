// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn has_len_a_times_b_and_contains_only_input_pairs(a in prop::collection::vec(0i8..8, 0..8), b in prop::collection::vec(0i8..8, 0..8)) {
        let out = cartesian_product(&a, &b);
        prop_assert_eq!(out.len(), a.len() * b.len());
        prop_assert!(out.iter().all(|(x, y)| a.contains(x) && b.contains(y)));
    }
}
