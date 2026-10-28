// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

fn style() -> impl Strategy<Value = FontStyle> {
    prop::sample::select(vec![
        FontStyle::Normal,
        FontStyle::Italic,
        FontStyle::Oblique,
    ])
}

fn face() -> impl Strategy<Value = FontFace> {
    (
        1u32..=1000,
        0u32..=300,
        style(),
        prop::sample::select(vec![50.0, 75.0, 100.0, 125.0]),
    )
        .prop_map(|(weight, extra, style, stretch)| {
            FontFace::variable(f64::from(weight), f64::from(weight + extra), style)
                .with_stretch(stretch)
        })
}

proptest! {
    #[test]
    fn picks_a_face_whenever_there_is_one(faces in prop::collection::vec(face(), 0..8), weight in 1u32..=1000, wanted_style in style()) {
        let picked = pick_closest(&faces, FontRequest::new(f64::from(weight), wanted_style));
        prop_assert_eq!(picked.is_some(), !faces.is_empty());
        if let Some(index) = picked {
            prop_assert!(index < faces.len());
        }
    }

    #[test]
    fn a_face_that_matches_everything_exactly_is_picked(faces in prop::collection::vec(face(), 0..8), weight in 1u32..=1000, wanted_style in style()) {
        let exact = FontFace::new(f64::from(weight), wanted_style);
        let mut all = faces;
        all.insert(0, exact);
        prop_assert_eq!(pick_closest(&all, FontRequest::new(f64::from(weight), wanted_style)), Some(0));
    }
}
