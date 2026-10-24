// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn matches_exactly_when_an_inclusion_matches_and_no_exclusion_does(
        included in prop::collection::vec("[ab]{1,3}", 0..4),
        excluded in prop::collection::vec("[ab]{1,3}", 0..4),
        text in "[ab]{0,3}",
    ) {
        let mut patterns: Vec<String> = included.clone();
        patterns.extend(excluded.iter().map(|p| format!("!{p}")));
        let refs: Vec<&str> = patterns.iter().map(String::as_str).collect();
        let expected = (included.is_empty() || included.contains(&text)) && !excluded.contains(&text);
        prop_assert_eq!(GlobSet::new(&refs).is_match(&text), expected);
    }

    #[test]
    fn the_order_of_the_patterns_does_not_matter(patterns in prop::collection::vec("!?[ab*?]{1,3}", 0..6), text in "[ab]{0,3}") {
        let refs: Vec<&str> = patterns.iter().map(String::as_str).collect();
        let reversed: Vec<&str> = refs.iter().rev().copied().collect();
        prop_assert_eq!(GlobSet::new(&refs).is_match(&text), GlobSet::new(&reversed).is_match(&text));
    }
}
