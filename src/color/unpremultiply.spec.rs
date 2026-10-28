// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use crate::color::premultiply;
use proptest::prelude::*;

proptest! {
    #[test]
    fn undoes_premultiply_within_the_rounding_of_a_translucent_color(r in any::<u8>(), a in 128u8..=255) {
        let back = unpremultiply(premultiply(Rgba::new(r, r, r, a)));
        // A premultiplied channel keeps `a` levels out of 255, so the error is at most 255 / a.
        prop_assert!(back.r().abs_diff(r) <= 2);
        prop_assert_eq!(back.a(), a);
    }
}
