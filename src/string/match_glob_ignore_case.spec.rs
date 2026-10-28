// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn agrees_with_match_glob_on_lower_case_input(pattern in "[a-z*?]{0,8}", text in "[a-z]{0,8}") {
        prop_assert_eq!(match_glob_ignore_case(&pattern, &text), match_glob(&pattern, &text));
    }

    #[test]
    fn the_case_of_the_text_never_matters(pattern in "[a-z*?]{0,8}", text in "[a-z]{0,8}") {
        prop_assert_eq!(
            match_glob_ignore_case(&pattern, &text),
            match_glob_ignore_case(&pattern, &text.to_uppercase())
        );
    }
}
