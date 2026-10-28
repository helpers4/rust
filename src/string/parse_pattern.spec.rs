// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics_and_a_regular_expression_reassembles(pattern in "\\PC{0,20}") {
        match parse_pattern(&pattern) {
            Pattern::RegExp { source, flags } => prop_assert_eq!(format!("/{source}/{flags}"), pattern),
            Pattern::Glob(glob) => prop_assert_eq!(glob, pattern),
        }
    }
}
