// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn has_the_same_length_and_reveals_at_most_a_quarter(secret in "\\PC{0,40}", visible in 0usize..50) {
        let masked = mask(&secret, visible);
        let length = secret.chars().count();
        prop_assert_eq!(masked.chars().count(), length);
        let shown = masked.chars().filter(|c| *c != '*').count();
        prop_assert!(shown <= length / 4);
    }
}
