// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics_and_ordinary_lowercase_words_are_never_tokens(s in "\\PC{0,60}", word in "[a-z]{1,20}") {
        let _ = detect(&s);
        prop_assert_eq!(detect(&word), None);
    }
}
