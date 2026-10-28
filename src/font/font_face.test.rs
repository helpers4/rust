// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

#![allow(clippy::float_cmp)]

use super::*;

#[test]
fn a_static_face_covers_one_weight() {
    let face = FontFace::new(700.0, FontStyle::Italic);
    assert_eq!(
        face,
        FontFace {
            weight_min: 700.0,
            weight_max: 700.0,
            style: FontStyle::Italic,
            stretch: 100.0
        }
    );
}

#[test]
fn a_variable_face_covers_a_range() {
    let face = FontFace::variable(100.0, 900.0, FontStyle::Normal);
    assert_eq!((face.weight_min, face.weight_max), (100.0, 900.0));
}

#[test]
fn with_stretch_changes_only_the_width() {
    let face = FontFace::new(400.0, FontStyle::Normal).with_stretch(75.0);
    assert_eq!(face.stretch, 75.0);
    assert_eq!(face.weight_min, 400.0);
}
