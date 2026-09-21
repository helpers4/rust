// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn parse_reads_back_what_format_wrote(millis in 0u64..10_000_000_000) {
        let d = Duration::from_millis(millis);
        prop_assert_eq!(crate::duration::parse(&format(d)), Ok(d));
    }

    #[test]
    fn never_has_edge_or_doubled_spaces(millis in 0u64..10_000_000_000) {
        let out = format(Duration::from_millis(millis));
        prop_assert!(!out.is_empty() && !out.starts_with(' ') && !out.ends_with(' ') && !out.contains("  "));
    }
}
