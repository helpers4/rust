// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn nothing_from_b_survives(a in prop::collection::vec(0i8..8, 0..20), b in prop::collection::vec(0i8..8, 0..20)) {
        prop_assert!(difference(&a, &b).iter().all(|x| !b.contains(x)));
    }

    #[test]
    fn result_is_a_filter_of_a(a in prop::collection::vec(0i8..8, 0..20), b in prop::collection::vec(0i8..8, 0..20)) {
        let expected: Vec<i8> = a.iter().filter(|x| !b.contains(x)).copied().collect();
        prop_assert_eq!(difference(&a, &b), expected);
    }

    #[test]
    fn difference_with_itself_is_empty(a in prop::collection::vec(0i8..8, 0..20)) {
        prop_assert!(difference(&a, &a).is_empty());
    }
}
