// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn the_output_only_contains_unreserved_characters_and_escapes(s in "\\PC*") {
        let out = percent_encode(&s);
        prop_assert!(out.bytes().all(|b| b.is_ascii_alphanumeric() || b"-._~%".contains(&b)));
    }

    #[test]
    fn decoding_gives_back_the_input(s in "\\PC*") {
        let encoded = percent_encode(&s);
        prop_assert_eq!(crate::url::percent_decode(&encoded).unwrap(), s.as_str());
    }
}
