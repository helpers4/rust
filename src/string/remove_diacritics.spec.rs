// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_panics(s in ".*") {
        let _ = remove_diacritics(&s);
    }

    #[test]
    fn is_idempotent(s in ".*") {
        let once = remove_diacritics(&s);
        let twice = remove_diacritics(&once);
        prop_assert_eq!(once, twice);
    }

    #[test]
    fn ascii_text_is_unchanged(s in "[a-zA-Z0-9 ]*") {
        // NFKD compatibility decomposition can *expand* some non-ASCII characters (ligatures,
        // fraction glyphs, circled digits, ...), so "never grows" only holds for plain ASCII,
        // which has no decomposition at all.
        prop_assert_eq!(remove_diacritics(&s), s);
    }
}
