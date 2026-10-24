// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use crate::array::toggle;
use proptest::prelude::*;

proptest! {
    #[test]
    fn with_the_identity_as_key_it_is_toggle(items in prop::collection::vec(0i32..10, 0..20), item in 0i32..10) {
        prop_assert_eq!(toggle_by(&items, item, |x| *x), toggle(&items, item));
    }
}
