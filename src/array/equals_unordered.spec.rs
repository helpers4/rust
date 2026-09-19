// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_reflexive_and_symmetric(a in prop::collection::vec(0i8..8, 0..20), b in prop::collection::vec(0i8..8, 0..20)) {
        prop_assert!(equals_unordered(&a, &a));
        prop_assert_eq!(equals_unordered(&a, &b), equals_unordered(&b, &a));
    }

    #[test]
    fn a_reversed_copy_is_equal(a in prop::collection::vec(0i8..8, 0..20)) {
        let mut reversed = a.clone();
        reversed.reverse();
        prop_assert!(equals_unordered(&a, &reversed));
    }

    #[test]
    fn agrees_with_comparing_sorted_copies(a in prop::collection::vec(0i8..4, 0..8), b in prop::collection::vec(0i8..4, 0..8)) {
        let (mut sa, mut sb) = (a.clone(), b.clone());
        sa.sort_unstable();
        sb.sort_unstable();
        prop_assert_eq!(equals_unordered(&a, &b), sa == sb);
    }
}
