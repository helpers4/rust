// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

#![allow(clippy::float_cmp)]

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics(a in ".*", b in ".*") {
        let _ = levenshtein_similarity(&a, &b, true);
    }

    #[test]
    fn stays_within_zero_and_one(a in "[a-z]{0,12}", b in "[a-z]{0,12}") {
        let score = levenshtein_similarity(&a, &b, true);
        prop_assert!((0.0..=1.0).contains(&score));
    }

    #[test]
    fn is_symmetric(a in "[a-z]{0,12}", b in "[a-z]{0,12}") {
        prop_assert_eq!(
            levenshtein_similarity(&a, &b, true),
            levenshtein_similarity(&b, &a, true)
        );
    }

    #[test]
    fn is_one_only_for_equal_strings(a in "[a-z]{0,12}", b in "[a-z]{0,12}") {
        prop_assert_eq!(levenshtein_similarity(&a, &b, true) == 1.0, a == b);
    }
}
