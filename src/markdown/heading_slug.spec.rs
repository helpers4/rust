// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn only_contains_lowercase_letters_digits_hyphens_and_underscores(s in "[ -~]{0,30}") {
        let slug = heading_slug(&s);
        prop_assert!(slug.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-' || c == '_'));
    }

    #[test]
    fn is_idempotent_on_ascii(s in "[ -~]{0,30}") {
        let once = heading_slug(&s);
        prop_assert_eq!(heading_slug(&once), once);
    }
}
