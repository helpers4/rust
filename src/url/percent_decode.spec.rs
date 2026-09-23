// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics_on_arbitrary_input(s in "\\PC*") {
        let _ = percent_decode(&s);
    }

    #[test]
    fn decoding_never_makes_the_text_longer(s in "[a-zA-Z0-9%]{0,30}") {
        if let Ok(decoded) = percent_decode(&s) {
            prop_assert!(decoded.len() <= s.len());
        }
    }
}
