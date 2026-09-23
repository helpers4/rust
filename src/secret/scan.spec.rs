// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn the_ranges_are_valid_ordered_and_never_overlap(text in "\\PC{0,80}") {
        let mut last_end = 0;
        for finding in scan(&text) {
            prop_assert!(finding.start() >= last_end && finding.start() < finding.end());
            prop_assert!(text.is_char_boundary(finding.start()) && text.is_char_boundary(finding.end()));
            last_end = finding.end();
        }
    }

    #[test]
    fn a_token_inside_ordinary_words_is_found_exactly(before in "[a-z ]{0,10}", after in "[a-z ]{0,10}") {
        let text = format!("{before} AKIAIOSFODNN7EXAMPLE {after}");
        let findings = scan(&text);
        prop_assert_eq!(findings.len(), 1);
        prop_assert_eq!(&text[findings[0].start()..findings[0].end()], "AKIAIOSFODNN7EXAMPLE");
    }
}
