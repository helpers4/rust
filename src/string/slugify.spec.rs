// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_idempotent_on_ascii(s in "[ -~]*") {
        let once = slugify(&s);
        prop_assert_eq!(slugify(&once), once);
    }

    #[test]
    fn ascii_output_is_url_safe(s in "[ -~]*") {
        let out = slugify(&s);
        prop_assert!(out.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-'));
        prop_assert!(!out.starts_with('-') && !out.ends_with('-') && !out.contains("--"));
    }
}
