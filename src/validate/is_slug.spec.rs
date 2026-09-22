// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// Cross-checks against `string::slugify`, so the whole file only compiles when that module is
// also enabled: `validate` must build and test on its own with no other feature turned on.
#![cfg(feature = "string")]

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn matches_slugify_on_ascii_input(s in "[ -~]{0,30}") {
        let slug = crate::string::slugify(&s);
        if !slug.is_empty() {
            prop_assert!(is_slug(&slug));
        }
    }
}
