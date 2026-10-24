// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    #[test]
    fn hex_round_trips(r in any::<u8>(), g in any::<u8>(), b in any::<u8>(), a in any::<u8>()) {
        let color = Rgba::new(r, g, b, a);
        prop_assert_eq!(Rgba::parse(&color.to_hex()), Ok(color));
    }

    #[test]
    fn css_round_trips_to_within_one_alpha_step(r in any::<u8>(), g in any::<u8>(), b in any::<u8>(), a in any::<u8>()) {
        // The opacity is written with 3 decimals: 1/255 is 0.0039, so the alpha comes back exactly.
        let color = Rgba::new(r, g, b, a);
        prop_assert_eq!(Rgba::parse(&color.to_css()), Ok(color));
    }

    #[test]
    fn opaque_colors_are_written_like_rgb(r in any::<u8>(), g in any::<u8>(), b in any::<u8>()) {
        let rgb = Rgb::new(r, g, b);
        prop_assert_eq!(Rgba::from(rgb).to_hex(), rgb.to_hex());
        prop_assert_eq!(Rgba::from(rgb).to_css(), rgb.to_css());
    }

    #[test]
    fn parse_never_panics(text in ".{0,40}") {
        let _ = Rgba::parse(&text);
    }
}
