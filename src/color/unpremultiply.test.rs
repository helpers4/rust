// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn divides_the_channels_by_the_alpha() {
    assert_eq!(
        unpremultiply(Rgba::new(100, 50, 25, 128)),
        Rgba::new(199, 100, 50, 128)
    );
}

#[test]
fn an_opaque_color_is_unchanged() {
    assert_eq!(
        unpremultiply(Rgba::new(200, 100, 50, 255)),
        Rgba::new(200, 100, 50, 255)
    );
}

#[test]
fn a_transparent_color_is_transparent_black() {
    assert_eq!(unpremultiply(Rgba::new(9, 9, 9, 0)), Rgba::new(0, 0, 0, 0));
}

#[test]
fn clamps_an_invalid_premultiplied_channel() {
    assert_eq!(
        unpremultiply(Rgba::new(255, 0, 0, 10)),
        Rgba::new(255, 0, 0, 10)
    );
}
