// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

const MODES: [BlendMode; 16] = [
    BlendMode::Normal,
    BlendMode::Multiply,
    BlendMode::Screen,
    BlendMode::Overlay,
    BlendMode::Darken,
    BlendMode::Lighten,
    BlendMode::ColorDodge,
    BlendMode::ColorBurn,
    BlendMode::HardLight,
    BlendMode::SoftLight,
    BlendMode::Difference,
    BlendMode::Exclusion,
    BlendMode::Hue,
    BlendMode::Saturation,
    BlendMode::Color,
    BlendMode::Luminosity,
];

fn color() -> impl Strategy<Value = Rgba> {
    (any::<u8>(), any::<u8>(), any::<u8>(), any::<u8>())
        .prop_map(|(r, g, b, a)| Rgba::new(r, g, b, a))
}

proptest! {
    #[test]
    fn never_panics_for_any_colors_and_mode(backdrop in color(), source in color(), mode in prop::sample::select(MODES.to_vec())) {
        let _ = blend(backdrop, source, mode);
    }

    #[test]
    fn an_opaque_source_replaces_the_backdrop_with_normal(backdrop in color(), r in any::<u8>(), g in any::<u8>(), b in any::<u8>()) {
        let source = Rgba::new(r, g, b, 255);
        prop_assert_eq!(blend(backdrop, source, BlendMode::Normal), source);
    }
}
