// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_idempotent_on_ascii(s in "[ -~]*") {
        let once = kebab_case(&s);
        prop_assert_eq!(kebab_case(&once), once);
    }
}
