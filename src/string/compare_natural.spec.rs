// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_reflexive_and_antisymmetric(a in "[a0-9 ]{0,8}", b in "[a0-9 ]{0,8}") {
        prop_assert_eq!(compare_natural(&a, &a), Ordering::Equal);
        prop_assert_eq!(compare_natural(&a, &b), compare_natural(&b, &a).reverse());
    }

    #[test]
    fn is_equal_only_for_identical_strings(a in "[a0-9]{0,8}", b in "[a0-9]{0,8}") {
        prop_assert_eq!(compare_natural(&a, &b) == Ordering::Equal, a == b);
    }

    #[test]
    fn is_transitive(a in "[a0-9]{0,6}", b in "[a0-9]{0,6}", c in "[a0-9]{0,6}") {
        if compare_natural(&a, &b) != Ordering::Greater && compare_natural(&b, &c) != Ordering::Greater {
            prop_assert_ne!(compare_natural(&a, &c), Ordering::Greater);
        }
    }

    #[test]
    fn orders_integers_like_integers(x in 0u64..1_000_000, y in 0u64..1_000_000) {
        prop_assert_eq!(compare_natural(&format!("n{x}"), &format!("n{y}")), x.cmp(&y));
    }
}
