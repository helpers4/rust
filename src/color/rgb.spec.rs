// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn hex_round_trips(r in any::<u8>(), g in any::<u8>(), b in any::<u8>()) {
        let color = Rgb::new(r, g, b);
        prop_assert_eq!(Rgb::parse(&color.to_hex()), Ok(color));
        prop_assert_eq!(Rgb::parse(&format!("rgb({r}, {g}, {b})")), Ok(color));
    }

    #[test]
    fn the_hsl_round_trip_gives_back_the_color(r in any::<u8>(), g in any::<u8>(), b in any::<u8>()) {
        let color = Rgb::new(r, g, b);
        prop_assert_eq!(color.to_hsl().to_rgb(), color);
    }

    #[test]
    fn hsl_values_stay_in_range(r in any::<u8>(), g in any::<u8>(), b in any::<u8>()) {
        let hsl = Rgb::new(r, g, b).to_hsl();
        prop_assert!((0.0..360.0).contains(&hsl.h()));
        prop_assert!((0.0..=1.0).contains(&hsl.s()));
        prop_assert!((0.0..=1.0).contains(&hsl.l()));
    }

    #[test]
    fn luminance_is_between_zero_and_one(r in any::<u8>(), g in any::<u8>(), b in any::<u8>()) {
        prop_assert!((0.0..=1.0 + 1e-12).contains(&Rgb::new(r, g, b).luminance()));
    }

    #[test]
    fn lighter_is_never_darker(r in any::<u8>(), g in any::<u8>(), b in any::<u8>(), amount in 0.0f64..1.0) {
        let color = Rgb::new(r, g, b);
        let lighter = color.lighten(amount);
        prop_assert!(lighter.r() >= color.r() && lighter.g() >= color.g() && lighter.b() >= color.b());
        let darker = color.darken(amount);
        prop_assert!(darker.r() <= color.r() && darker.g() <= color.g() && darker.b() <= color.b());
    }

    #[test]
    fn parsing_never_panics(s in "\\PC{0,15}") {
        let _ = Rgb::parse(&s);
    }
}
