// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn gives_back_every_value_in_order(delays in prop::collection::vec(0u32..4, 0..8)) {
        let futures: Vec<_> = delays
            .iter()
            .enumerate()
            .map(|(i, delay)| crate::future::countdown::countdown(*delay, u32::try_from(i).unwrap()))
            .collect();
        let expected: Vec<u32> = (0..delays.len()).map(|i| u32::try_from(i).unwrap()).collect();
        prop_assert_eq!(crate::future::block_on(join_all(futures)), expected);
    }
}
