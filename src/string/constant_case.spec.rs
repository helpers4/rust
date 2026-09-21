// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_the_upper_case_of_snake_case(s in "[ -~]*") {
        prop_assert_eq!(constant_case(&s), crate::string::snake_case(&s).to_uppercase());
    }

    #[test]
    fn only_contains_upper_case_digits_and_underscores(s in "[ -~]*") {
        let out = constant_case(&s);
        prop_assert!(out.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_'));
        prop_assert!(!out.starts_with('_') && !out.ends_with('_') && !out.contains("__"));
    }
}
