// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn the_destination_never_contains_a_character_that_ends_it(url in "[ -~]{0,20}") {
        let out = link("t", &url);
        let destination = &out[4..out.len() - 1];
        prop_assert!(!destination.contains([' ', '(', ')', '<', '>', '\n']));
    }
}
