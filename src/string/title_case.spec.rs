// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn is_idempotent_on_ascii(s in "[ -~]*") {
        let once = title_case(&s);
        prop_assert_eq!(title_case(&once), once);
    }

    #[test]
    fn only_changes_the_case_of_ascii_text(s in "[ -~]*") {
        let out = title_case(&s);
        prop_assert_eq!(out.len(), s.len());
        prop_assert_eq!(out.to_lowercase(), s.to_lowercase());
    }
}
