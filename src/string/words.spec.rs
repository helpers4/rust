// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics(s in ".*") {
        let _ = words(&s);
    }

    #[test]
    fn every_word_is_non_empty_and_alphanumeric(s in ".*") {
        for word in words(&s) {
            prop_assert!(!word.is_empty());
            prop_assert!(word.chars().all(char::is_alphanumeric));
        }
    }

    #[test]
    fn agrees_with_lowercasing_every_word_of_the_internal_case_splitter(s in "[a-zA-Z0-9 _-]{0,30}") {
        let lowercased: Vec<String> = words(&s).into_iter().map(str::to_lowercase).collect();
        prop_assert_eq!(lowercased, super::super::_words::words(&s));
    }
}
