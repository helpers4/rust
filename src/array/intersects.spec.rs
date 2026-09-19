// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_symmetric(a in prop::collection::vec(0i8..8, 0..20), b in prop::collection::vec(0i8..8, 0..20)) {
        prop_assert_eq!(intersects(&a, &b), intersects(&b, &a));
    }

    #[test]
    fn agrees_with_a_naive_search(a in prop::collection::vec(0i8..8, 0..20), b in prop::collection::vec(0i8..8, 0..20)) {
        prop_assert_eq!(intersects(&a, &b), a.iter().any(|x| b.contains(x)));
    }
}
