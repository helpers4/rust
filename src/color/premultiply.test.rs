// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn scales_the_channels_by_the_alpha() {
    assert_eq!(
        premultiply(Rgba::new(200, 100, 50, 128)),
        Rgba::new(100, 50, 25, 128)
    );
}

#[test]
fn an_opaque_color_is_unchanged() {
    assert_eq!(
        premultiply(Rgba::new(200, 100, 50, 255)),
        Rgba::new(200, 100, 50, 255)
    );
}

#[test]
fn a_transparent_color_loses_its_channels() {
    assert_eq!(
        premultiply(Rgba::new(200, 100, 50, 0)),
        Rgba::new(0, 0, 0, 0)
    );
}
