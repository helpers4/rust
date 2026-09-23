// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_leaves_an_escape_character_that_started_a_known_sequence(s in "[a-z \\[;0-9m]{0,20}") {
        let text = format!("\u{1b}[31m{s}\u{1b}[0m");
        let out = strip(&text);
        let has_escape = out.contains('\u{1b}');
        prop_assert!(!has_escape);
    }

    #[test]
    fn text_without_escapes_is_unchanged(s in "[^\\u{1b}]{0,30}") {
        prop_assert_eq!(strip(&s), s.as_str());
    }

    #[test]
    fn never_panics(s in "\\PC{0,30}\u{1b}?[\\[\\]0-9;a-z\\u{7}\\\\]{0,10}") {
        let _ = strip(&s);
    }
}
