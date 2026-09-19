// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_idempotent_on_ascii(s in "[ -~]*") {
        let once = snake_case(&s);
        prop_assert_eq!(snake_case(&once), once);
    }

    #[test]
    fn has_no_edge_or_doubled_separators(s in "[ -~]*") {
        let out = snake_case(&s);
        prop_assert!(!out.starts_with('_') && !out.ends_with('_') && !out.contains("__"));
    }
}
