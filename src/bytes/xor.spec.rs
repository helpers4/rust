// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn xoring_twice_gives_back_the_input(
        pairs in prop::collection::vec((any::<u8>(), any::<u8>()), 0..16),
    ) {
        let (a, b): (Vec<u8>, Vec<u8>) = pairs.into_iter().unzip();
        let once = xor(&a, &b).unwrap();
        prop_assert_eq!(xor(&once, &b).unwrap(), a);
    }
}
