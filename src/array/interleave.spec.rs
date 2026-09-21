// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn keeps_every_element_and_each_side_in_order(
        a in prop::collection::vec(0i32..100, 0..20),
        b in prop::collection::vec(100i32..200, 0..20),
    ) {
        let out = interleave(&a, &b);
        prop_assert_eq!(out.len(), a.len() + b.len());
        prop_assert_eq!(out.iter().copied().filter(|x| *x < 100).collect::<Vec<_>>(), a);
        prop_assert_eq!(out.iter().copied().filter(|x| *x >= 100).collect::<Vec<_>>(), b);
    }
}
