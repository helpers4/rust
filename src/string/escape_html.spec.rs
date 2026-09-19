// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn output_has_no_raw_specials(s in ".*") {
        let out = escape_html(&s);
        prop_assert!(!out.contains(['<', '>', '"', '\'']));
    }

    #[test]
    fn borrowed_iff_input_has_no_specials(s in ".*") {
        let has_special = s.contains(['&', '<', '>', '"', '\'']);
        prop_assert_eq!(matches!(escape_html(&s), Cow::Owned(_)), has_special);
    }
}
