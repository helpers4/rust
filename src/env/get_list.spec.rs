// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics(content in ".*", key in "[A-Za-z_][A-Za-z0-9_]*") {
        let _ = get_list(&content, &key, ',', &[]);
    }

    #[test]
    fn item_count_is_separator_count_plus_one(
        items in prop::collection::vec("[a-z]{0,5}", 1..8),
    ) {
        let joined = items.join(",");
        let content = format!("KEY={joined}");
        prop_assert_eq!(get_list(&content, "KEY", ',', &[]).len(), items.len());
    }
}
