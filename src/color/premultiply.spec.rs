// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn never_raises_a_channel_and_keeps_the_alpha(r in any::<u8>(), g in any::<u8>(), b in any::<u8>(), a in any::<u8>()) {
        let out = premultiply(Rgba::new(r, g, b, a));
        prop_assert!(out.r() <= r && out.g() <= g && out.b() <= b);
        prop_assert_eq!(out.a(), a);
    }
}
