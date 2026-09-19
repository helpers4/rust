// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_exceeds_the_limit(s in ".*", max in 0usize..40, suffix in ".{0,5}") {
        prop_assert!(truncate(&s, max, &suffix).chars().count() <= max);
    }

    #[test]
    fn is_identity_when_input_fits(s in ".{0,20}", suffix in ".{0,5}") {
        let max = s.chars().count();
        prop_assert_eq!(truncate(&s, max, &suffix), s);
    }
}
