// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn the_block_opens_and_closes_with_a_fence_longer_than_any_run_in_the_code(code in "[a-c`\\n]{0,30}") {
        let block = code_block(&code, "x");
        let fence = block.chars().take_while(|c| *c == '`').count();
        let longest = code.split(|c| c != '`').map(str::len).max().unwrap_or(0);
        prop_assert!(fence >= 3 && fence > longest);
        let closing = format!("\n{}\n", "`".repeat(fence));
        prop_assert!(block.ends_with(&closing));
    }
}
