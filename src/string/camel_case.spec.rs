// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

// Not idempotent by design: consecutive single-letter words (`A0A` -> `a0AA`) merge into an
// acronym run on the next pass, so only output shape is checked here.
proptest! {
    #[test]
    fn has_no_separators_and_starts_lowercase(s in "[ -~]*") {
        let out = camel_case(&s);
        prop_assert!(out.chars().all(|c| c.is_ascii_alphanumeric()));
        prop_assert!(!out.starts_with(|c: char| c.is_ascii_uppercase()));
    }
}
