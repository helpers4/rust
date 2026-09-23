// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn every_line_has_the_same_number_of_cells(
        headers in prop::collection::vec("[a-z]{0,5}", 1..4),
        rows in prop::collection::vec(prop::collection::vec("[a-z]{0,5}", 0..5), 0..4),
    ) {
        let out = table(&headers, &rows);
        let lines: Vec<&str> = out.lines().collect();
        prop_assert_eq!(lines.len(), rows.len() + 2);
        let width = lines[0].chars().count();
        for line in lines {
            prop_assert!(line.starts_with("| ") && line.ends_with(" |"));
            prop_assert_eq!(line.matches(" | ").count(), headers.len() - 1);
            prop_assert_eq!(line.chars().count(), width);
        }
    }
}
