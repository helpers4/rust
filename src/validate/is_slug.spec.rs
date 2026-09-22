// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

// Cross-checks against `string::slugify`, so it only runs when that module is also enabled:
// `validate` must compile and pass its own tests with no other feature turned on.
#[cfg(feature = "string")]
proptest! {
    #[test]
    fn matches_slugify_on_ascii_input(s in "[ -~]{0,30}") {
        let slug = crate::string::slugify(&s);
        if !slug.is_empty() {
            prop_assert!(is_slug(&slug));
        }
    }
}
