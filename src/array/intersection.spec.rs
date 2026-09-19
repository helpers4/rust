// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn every_element_is_in_both(a in prop::collection::vec(0i8..8, 0..20), b in prop::collection::vec(0i8..8, 0..20)) {
        prop_assert!(intersection(&a, &b).iter().all(|x| a.contains(x) && b.contains(x)));
    }

    #[test]
    fn splits_a_together_with_difference(a in prop::collection::vec(0i8..8, 0..20), b in prop::collection::vec(0i8..8, 0..20)) {
        prop_assert_eq!(intersection(&a, &b).len() + difference_len(&a, &b), a.len());
    }

    #[test]
    fn intersection_with_itself_is_identity(a in prop::collection::vec(0i8..8, 0..20)) {
        prop_assert_eq!(intersection(&a, &a), a);
    }
}

fn difference_len(a: &[i8], b: &[i8]) -> usize {
    a.iter().filter(|x| !b.contains(x)).count()
}
