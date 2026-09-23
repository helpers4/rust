// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn each_channel_stays_between_the_two_inputs(
        a in (any::<u8>(), any::<u8>(), any::<u8>()),
        b in (any::<u8>(), any::<u8>(), any::<u8>()),
        t in 0.0f64..=1.0,
    ) {
        let (a, b) = (Rgb::new(a.0, a.1, a.2), Rgb::new(b.0, b.1, b.2));
        let m = mix(a, b, t);
        prop_assert!(m.r() >= a.r().min(b.r()) && m.r() <= a.r().max(b.r()));
        prop_assert!(m.g() >= a.g().min(b.g()) && m.g() <= a.g().max(b.g()));
        prop_assert!(m.b() >= a.b().min(b.b()) && m.b() <= a.b().max(b.b()));
    }
}
