// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

#![allow(clippy::float_cmp)]

use super::*;

fn info() -> FontInfo {
    FontInfo {
        family: "Inter".into(),
        style: "Bold".into(),
        postscript_name: None,
        weight: 700.0,
        weight_range: None,
        italic: false,
    }
}

#[test]
fn a_static_font_is_a_face_of_one_weight() {
    assert_eq!(info().to_face(), FontFace::new(700.0, FontStyle::Normal));
}

#[test]
fn an_italic_font_is_an_italic_face() {
    let italic = FontInfo {
        italic: true,
        ..info()
    };
    assert_eq!(italic.to_face().style, FontStyle::Italic);
}

#[test]
fn a_variable_font_covers_its_range() {
    let variable = FontInfo {
        weight_range: Some((100.0, 900.0)),
        ..info()
    };
    let face = variable.to_face();
    assert_eq!((face.weight_min, face.weight_max), (100.0, 900.0));
}
