// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics(a in ".*", b in ".*") {
        let _ = levenshtein_distance(&a, &b, true);
    }

    #[test]
    fn is_symmetric(a in "[a-z]{0,12}", b in "[a-z]{0,12}") {
        prop_assert_eq!(levenshtein_distance(&a, &b, true), levenshtein_distance(&b, &a, true));
    }

    #[test]
    fn is_zero_only_for_equal_strings(a in "[a-z]{0,12}", b in "[a-z]{0,12}") {
        prop_assert_eq!(levenshtein_distance(&a, &b, true) == 0, a == b);
    }

    #[test]
    fn is_at_most_the_length_of_the_longer_string(a in "[a-z]{0,12}", b in "[a-z]{0,12}") {
        prop_assert!(levenshtein_distance(&a, &b, true) <= a.chars().count().max(b.chars().count()));
    }
}
