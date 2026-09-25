// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics(text in ".*", max_chars in 0usize..40) {
        let _ = excerpt(&text, max_chars, "…");
    }

    #[test]
    fn never_exceeds_max_chars(text in ".{0,80}", max_chars in 1usize..40) {
        prop_assert!(excerpt(&text, max_chars, "…").chars().count() <= max_chars);
    }

    #[test]
    fn is_the_identity_when_the_squished_text_already_fits(text in "[a-zA-Z ]{0,20}") {
        let collapsed = super::super::squish::squish(&text);
        prop_assert_eq!(excerpt(&text, collapsed.chars().count().max(1), "…"), collapsed);
    }
}
