// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn face(weight: f64, style: FontStyle) -> FontFace {
    FontFace::new(weight, style)
}

fn pick(faces: &[FontFace], weight: f64, style: FontStyle) -> Option<usize> {
    pick_closest(faces, FontRequest::new(weight, style))
}

#[test]
fn no_face_gives_none() {
    assert_eq!(pick(&[], 400.0, FontStyle::Normal), None);
}

#[test]
fn an_exact_match_wins() {
    let faces = [
        face(300.0, FontStyle::Normal),
        face(400.0, FontStyle::Normal),
        face(700.0, FontStyle::Normal),
    ];
    assert_eq!(pick(&faces, 400.0, FontStyle::Normal), Some(1));
    assert_eq!(pick(&faces, 700.0, FontStyle::Normal), Some(2));
}

#[test]
fn above_500_heavier_first_then_lighter() {
    let faces = [
        face(300.0, FontStyle::Normal),
        face(400.0, FontStyle::Normal),
        face(700.0, FontStyle::Normal),
    ];
    assert_eq!(pick(&faces, 600.0, FontStyle::Normal), Some(2));
    assert_eq!(pick(&faces, 800.0, FontStyle::Normal), Some(2));
    let light = [
        face(300.0, FontStyle::Normal),
        face(400.0, FontStyle::Normal),
    ];
    assert_eq!(pick(&light, 800.0, FontStyle::Normal), Some(1));
}

#[test]
fn below_400_lighter_first_then_heavier() {
    let faces = [
        face(200.0, FontStyle::Normal),
        face(300.0, FontStyle::Normal),
        face(500.0, FontStyle::Normal),
    ];
    assert_eq!(pick(&faces, 250.0, FontStyle::Normal), Some(0));
    assert_eq!(pick(&faces, 100.0, FontStyle::Normal), Some(0));
    let heavy = [
        face(500.0, FontStyle::Normal),
        face(700.0, FontStyle::Normal),
    ];
    assert_eq!(pick(&heavy, 100.0, FontStyle::Normal), Some(0));
}

#[test]
fn between_400_and_500_looks_up_to_500_then_down_then_above() {
    let faces = [
        face(300.0, FontStyle::Normal),
        face(500.0, FontStyle::Normal),
        face(700.0, FontStyle::Normal),
    ];
    assert_eq!(pick(&faces, 450.0, FontStyle::Normal), Some(1));
    let no_500 = [
        face(300.0, FontStyle::Normal),
        face(700.0, FontStyle::Normal),
    ];
    assert_eq!(pick(&no_500, 450.0, FontStyle::Normal), Some(0));
    let only_heavy = [
        face(600.0, FontStyle::Normal),
        face(900.0, FontStyle::Normal),
    ];
    assert_eq!(pick(&only_heavy, 450.0, FontStyle::Normal), Some(0));
}

#[test]
fn style_is_matched_before_weight() {
    let faces = [
        face(400.0, FontStyle::Normal),
        face(700.0, FontStyle::Italic),
        face(400.0, FontStyle::Oblique),
    ];
    assert_eq!(pick(&faces, 400.0, FontStyle::Italic), Some(1));
    assert_eq!(pick(&faces, 400.0, FontStyle::Normal), Some(0));
    assert_eq!(pick(&faces, 400.0, FontStyle::Oblique), Some(2));
}

#[test]
fn style_falls_back_in_the_css_order() {
    let normal_italic = [
        face(400.0, FontStyle::Normal),
        face(400.0, FontStyle::Italic),
    ];
    assert_eq!(pick(&normal_italic[1..], 400.0, FontStyle::Normal), Some(0));
    let oblique_only = [
        face(400.0, FontStyle::Oblique),
        face(400.0, FontStyle::Italic),
    ];
    assert_eq!(pick(&oblique_only, 400.0, FontStyle::Normal), Some(0));
    assert_eq!(pick(&oblique_only, 400.0, FontStyle::Italic), Some(1));
    let italic_normal = [
        face(400.0, FontStyle::Normal),
        face(400.0, FontStyle::Italic),
    ];
    assert_eq!(pick(&italic_normal, 400.0, FontStyle::Oblique), Some(1));
    assert_eq!(pick(&italic_normal[..1], 400.0, FontStyle::Italic), Some(0));
}

#[test]
fn stretch_is_matched_first() {
    let faces = [
        face(700.0, FontStyle::Normal).with_stretch(100.0),
        face(400.0, FontStyle::Normal).with_stretch(75.0),
        face(400.0, FontStyle::Normal).with_stretch(125.0),
    ];
    let request = |stretch: f64| FontRequest {
        stretch,
        ..FontRequest::default()
    };
    assert_eq!(pick_closest(&faces, request(75.0)), Some(1));
    assert_eq!(pick_closest(&faces, request(100.0)), Some(0));
    assert_eq!(pick_closest(&faces, request(125.0)), Some(2));
    // Narrower than wanted first when at or below normal, wider first above it.
    assert_eq!(pick_closest(&faces[1..], request(90.0)), Some(0));
    assert_eq!(pick_closest(&faces[1..2], request(90.0)), Some(0));
    assert_eq!(pick_closest(&faces[..1], request(150.0)), Some(0));
    assert_eq!(pick_closest(&faces[1..], request(110.0)), Some(1));
    assert_eq!(pick_closest(&faces[1..], request(140.0)), Some(1));
    assert_eq!(pick_closest(&faces[1..2], request(140.0)), Some(0));
}

#[test]
fn the_first_of_equally_close_faces_wins() {
    let faces = [
        face(400.0, FontStyle::Normal),
        face(400.0, FontStyle::Normal),
    ];
    assert_eq!(pick(&faces, 400.0, FontStyle::Normal), Some(0));
}

#[test]
fn a_variable_face_covering_the_weight_is_an_exact_match() {
    let faces = [
        face(600.0, FontStyle::Normal),
        FontFace::variable(100.0, 900.0, FontStyle::Normal),
    ];
    assert_eq!(pick(&faces, 450.0, FontStyle::Normal), Some(1));
    assert_eq!(pick(&faces, 600.0, FontStyle::Normal), Some(0));
}

#[test]
fn a_variable_face_outside_the_weight_counts_for_its_nearest_bound() {
    let faces = [
        FontFace::variable(100.0, 300.0, FontStyle::Normal),
        FontFace::variable(600.0, 900.0, FontStyle::Normal),
    ];
    assert_eq!(pick(&faces, 700.0, FontStyle::Normal), Some(1));
    assert_eq!(pick(&faces, 950.0, FontStyle::Normal), Some(1));
    assert_eq!(pick(&faces, 50.0, FontStyle::Normal), Some(0));
    assert_eq!(pick(&faces, 400.0, FontStyle::Normal), Some(0));
}

#[test]
fn a_reversed_range_is_read_as_its_lower_bound() {
    let faces = [FontFace::variable(500.0, 300.0, FontStyle::Normal)];
    assert_eq!(pick(&faces, 900.0, FontStyle::Normal), Some(0));
}
