// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics(content in ".*", key in "[A-Za-z_][A-Za-z0-9_]*", default in any::<i64>()) {
        let _ = get_int(&content, &key, default);
    }

    #[test]
    fn any_i64_round_trips(key in "[A-Za-z_][A-Za-z0-9_]*", value in any::<i64>()) {
        let content = format!("{key}={value}");
        prop_assert_eq!(get_int(&content, &key, 0), value);
    }
}
