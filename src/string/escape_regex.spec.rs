// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics(s in ".*") {
        let _ = escape_regex(&s);
    }

    #[test]
    fn removing_the_backslashes_it_added_gives_back_the_input(s in "[^\\\\]*") {
        // `s` has no backslash of its own, so every backslash in the output was inserted right
        // before a metacharacter: dropping every backslash undoes the escaping exactly.
        let escaped = escape_regex(&s);
        prop_assert_eq!(escaped.chars().filter(|&c| c != '\\').collect::<String>(), s);
    }

    #[test]
    fn output_is_never_shorter(s in ".*") {
        prop_assert!(escape_regex(&s).len() >= s.len());
    }
}
