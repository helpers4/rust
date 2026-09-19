// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_idempotent(s in ".*") {
        let once = capitalize(&s);
        prop_assert_eq!(capitalize(&once), once);
    }

    #[test]
    fn preserves_tail(s in ".+") {
        let first_len = s.chars().next().unwrap().len_utf8();
        let out = capitalize(&s);
        prop_assert!(out.ends_with(&s[first_len..]));
    }
}
