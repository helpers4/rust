// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn keeps_the_line_count(s in "[a-z \\n]*", n in 0usize..4) {
        let out = indent(&s, &" ".repeat(n));
        prop_assert_eq!(out.matches('\n').count(), s.matches('\n').count());
    }

    #[test]
    fn dedent_undoes_it(
        words in prop::collection::vec("[a-z]{1,5}", 1..5),
        n in 1usize..5,
    ) {
        let text = words.join("\n");
        prop_assert_eq!(crate::string::dedent(&indent(&text, &" ".repeat(n))), text);
    }
}
