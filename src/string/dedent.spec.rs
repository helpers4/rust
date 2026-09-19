// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn removes_exactly_the_shared_indent(
        words in prop::collection::vec("[a-z]{1,5}", 1..5),
        n in 0usize..6,
    ) {
        let pad = " ".repeat(n);
        let indented = words.iter().map(|w| format!("{pad}{w}")).collect::<Vec<_>>().join("\n");
        prop_assert_eq!(dedent(&indented), words.join("\n"));
    }
}
