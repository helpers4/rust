// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn agrees_with_slice_equality(
        a in prop::collection::vec(any::<u8>(), 0..16),
        b in prop::collection::vec(any::<u8>(), 0..16),
    ) {
        prop_assert_eq!(constant_time_eq(&a, &b), a == b);
        prop_assert!(constant_time_eq(&a, &a));
    }
}
