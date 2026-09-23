// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn stays_between_one_and_twenty_one_and_is_symmetric(
        a in (any::<u8>(), any::<u8>(), any::<u8>()),
        b in (any::<u8>(), any::<u8>(), any::<u8>()),
    ) {
        let (a, b) = (Rgb::new(a.0, a.1, a.2), Rgb::new(b.0, b.1, b.2));
        let ratio = contrast_ratio(a, b);
        prop_assert!((1.0..=21.0 + 1e-9).contains(&ratio));
        prop_assert!((ratio - contrast_ratio(b, a)).abs() < 1e-12);
    }
}
