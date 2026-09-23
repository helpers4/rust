// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn contains_exactly_the_elements_of_the_inputs(
        a in prop::collection::hash_set(0u8..30, 0..10),
        b in prop::collection::hash_set(0u8..30, 0..10),
    ) {
        let all = union_all(&[a.clone(), b.clone()]);
        for x in 0u8..30 {
            prop_assert_eq!(all.contains(&x), a.contains(&x) || b.contains(&x));
        }
    }
}
