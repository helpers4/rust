// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn every_line_starts_with_the_marker(text in "[a-z \\n]{0,30}") {
        let quoted = blockquote(&text);
        prop_assert!(quoted.split('\n').all(|line| line == ">" || line.starts_with("> ")));
        prop_assert_eq!(quoted.matches('\n').count(), text.matches('\n').count());
    }
}
