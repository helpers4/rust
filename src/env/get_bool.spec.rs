// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics(content in ".*", key in "[A-Za-z_][A-Za-z0-9_]*", default in any::<bool>()) {
        let _ = get_bool(&content, &key, default);
    }

    #[test]
    fn a_recognized_word_never_falls_back_to_the_default_of_the_opposite_value(
        word in prop::sample::select(vec!["true", "1", "yes", "on"]),
    ) {
        // Whatever `default` is, a truthy word must read back `true`.
        let content = format!("KEY={word}");
        prop_assert!(get_bool(&content, "KEY", false));
    }
}
