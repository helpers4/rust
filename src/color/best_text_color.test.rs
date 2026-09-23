// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn light_backgrounds_get_black_text() {
    assert_eq!(best_text_color(Rgb::new(255, 255, 255)), Rgb::new(0, 0, 0));
    assert_eq!(best_text_color(Rgb::new(255, 255, 0)), Rgb::new(0, 0, 0));
    assert_eq!(best_text_color(Rgb::new(255, 0, 0)), Rgb::new(0, 0, 0));
}

#[test]
fn dark_backgrounds_get_white_text() {
    assert_eq!(best_text_color(Rgb::new(0, 0, 0)), Rgb::new(255, 255, 255));
    assert_eq!(
        best_text_color(Rgb::new(0, 0, 255)),
        Rgb::new(255, 255, 255)
    );
    assert_eq!(
        best_text_color(Rgb::new(0, 0, 128)),
        Rgb::new(255, 255, 255)
    );
}
