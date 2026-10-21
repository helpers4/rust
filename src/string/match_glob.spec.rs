// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

/// Reference implementation: plain recursion over the definition.
fn reference(pattern: &[char], text: &[char]) -> bool {
    match pattern.split_first() {
        None => text.is_empty(),
        Some(('*', rest)) => (0..=text.len()).any(|i| reference(rest, &text[i..])),
        Some((&c, rest)) => text
            .split_first()
            .is_some_and(|(&t, tail)| (c == '?' || c == t) && reference(rest, tail)),
    }
}

proptest! {
    #[test]
    fn agrees_with_the_reference(pattern in "[ab*?]{0,8}", text in "[ab]{0,10}") {
        let p: Vec<char> = pattern.chars().collect();
        let t: Vec<char> = text.chars().collect();
        prop_assert_eq!(match_glob(&pattern, &text), reference(&p, &t));
    }

    #[test]
    fn a_text_matches_itself_and_prefix_star(text in "[^*?]{0,20}", rest in ".{0,10}") {
        prop_assert!(match_glob(&text, &text));
        let pattern = format!("{text}*");
        let extended = format!("{text}{rest}");
        prop_assert!(match_glob(&pattern, &extended));
    }
}
