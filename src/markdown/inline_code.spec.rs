// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn the_fence_is_longer_than_any_backtick_run_in_the_text(s in "[a-c` ]{0,20}") {
        let span = inline_code(&s);
        let fence = span.chars().take_while(|c| *c == '`').count();
        prop_assert!(fence > longest_backtick_run(&s));
        prop_assert!(span.ends_with(&"`".repeat(fence)));
    }
}
