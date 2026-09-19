// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_symmetric_as_a_set(a in prop::collection::vec(0i8..8, 0..20), b in prop::collection::vec(0i8..8, 0..20)) {
        let mut a_then_b = symmetric_difference(&a, &b);
        let mut b_then_a = symmetric_difference(&b, &a);
        a_then_b.sort_unstable();
        b_then_a.sort_unstable();
        prop_assert_eq!(a_then_b, b_then_a);
    }

    #[test]
    fn with_itself_is_empty(a in prop::collection::vec(0i8..8, 0..20)) {
        prop_assert!(symmetric_difference(&a, &a).is_empty());
    }

    #[test]
    fn every_element_is_in_exactly_one_side(a in prop::collection::vec(0i8..8, 0..20), b in prop::collection::vec(0i8..8, 0..20)) {
        prop_assert!(symmetric_difference(&a, &b).iter().all(|x| a.contains(x) != b.contains(x)));
    }
}
