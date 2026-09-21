// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn undoes_escape_html(s in "\\PC*") {
        let escaped = crate::string::escape_html(&s);
        prop_assert_eq!(unescape_html(&escaped), s.as_str());
    }

    #[test]
    fn never_panics_and_never_grows(s in "[&#x;a-z0-9A-F]{0,40}") {
        prop_assert!(unescape_html(&s).len() <= s.len());
    }
}
