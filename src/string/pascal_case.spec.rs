// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    // Not idempotent by design (see camel_case): only output shape is checked.
    #[test]
    fn has_no_separators(s in "[ -~]*") {
        prop_assert!(pascal_case(&s).chars().all(|c| c.is_ascii_alphanumeric()));
    }
}
