// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_idempotent(s in "\\PC*") {
        let once = squish(&s);
        prop_assert_eq!(squish(&once), once);
    }

    #[test]
    fn never_leaves_edge_or_doubled_spaces(s in "\\PC*") {
        let out = squish(&s);
        prop_assert!(!out.starts_with(' ') && !out.ends_with(' ') && !out.contains("  "));
    }
}
