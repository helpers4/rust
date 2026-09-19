// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics_and_only_returns_valid_keys(content in ".*") {
        for (key, _) in parse(&content) {
            prop_assert!(crate::env::_line::is_valid_key(&key));
        }
    }
}
