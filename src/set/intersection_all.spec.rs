// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn contains_exactly_what_every_input_contains(
        a in prop::collection::hash_set(0u8..30, 0..12),
        b in prop::collection::hash_set(0u8..30, 0..12),
    ) {
        let common = intersection_all(&[a.clone(), b.clone()]);
        for x in 0u8..30 {
            prop_assert_eq!(common.contains(&x), a.contains(&x) && b.contains(&x));
        }
    }
}
