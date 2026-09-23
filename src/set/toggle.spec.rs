// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn toggling_twice_restores_the_set(
        set in prop::collection::hash_set(0u8..20, 0..10),
        item in 0u8..20,
    ) {
        let mut copy = set.clone();
        toggle(&mut copy, item);
        toggle(&mut copy, item);
        prop_assert_eq!(copy, set);
    }
}
