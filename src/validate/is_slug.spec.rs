// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

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
