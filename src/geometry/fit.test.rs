// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

const WIDE: Size = Size::new(200.0, 100.0);
const SQUARE: Size = Size::new(100.0, 100.0);

fn m(a: f64, d: f64, e: f64, f: f64) -> Matrix {
    Matrix::new(a, 0.0, 0.0, d, e, f)
}

#[test]
fn contains_and_centers_by_default() {
    assert_eq!(
        fit(WIDE, SQUARE, FitMode::default(), FitAlign::default()),
        m(0.5, 0.5, 0.0, 25.0)
    );
}

#[test]
fn covers_overflowing_on_two_sides() {
    assert_eq!(
        fit(WIDE, SQUARE, FitMode::Cover, FitAlign::CENTER),
        m(1.0, 1.0, -50.0, 0.0)
    );
}

#[test]
fn fills_each_axis() {
    assert_eq!(
        fit(WIDE, SQUARE, FitMode::Fill, FitAlign::CENTER),
        m(0.5, 1.0, 0.0, 0.0)
    );
}

#[test]
fn keeps_the_original_size() {
    assert_eq!(
        fit(WIDE, SQUARE, FitMode::Original, FitAlign::CENTER),
        m(1.0, 1.0, -50.0, 0.0)
    );
}

#[test]
fn scale_down_shrinks_but_never_enlarges() {
    assert_eq!(
        fit(WIDE, SQUARE, FitMode::ScaleDown, FitAlign::CENTER),
        m(0.5, 0.5, 0.0, 25.0)
    );
    assert_eq!(
        fit(
            Size::new(10.0, 10.0),
            SQUARE,
            FitMode::ScaleDown,
            FitAlign::CENTER
        ),
        m(1.0, 1.0, 45.0, 45.0)
    );
}

#[test]
fn scales_up_small_content_with_contain() {
    assert_eq!(
        fit(
            Size::new(10.0, 20.0),
            SQUARE,
            FitMode::Contain,
            FitAlign::CENTER
        ),
        m(5.0, 5.0, 25.0, 0.0)
    );
}

#[test]
fn aligns_in_the_leftover_space_without_writing_negative_zero() {
    assert_eq!(
        fit(WIDE, SQUARE, FitMode::Contain, FitAlign::new(0.5, 0.0)),
        m(0.5, 0.5, 0.0, 0.0)
    );
    assert_eq!(
        fit(WIDE, SQUARE, FitMode::Contain, FitAlign::new(0.5, 1.0)),
        m(0.5, 0.5, 0.0, 50.0)
    );
    let left = fit(WIDE, SQUARE, FitMode::Cover, FitAlign::new(0.0, 0.5));
    assert_eq!(left, m(1.0, 1.0, 0.0, 0.0));
    assert!(left.e.is_sign_positive());
    assert!((fit(WIDE, SQUARE, FitMode::Cover, FitAlign::new(1.0, 0.5)).e + 100.0).abs() < 1e-12);
}

#[test]
fn a_vertical_overflow_aligned_to_the_top_is_a_positive_zero_too() {
    let tall = Size::new(100.0, 200.0);
    let top = fit(tall, SQUARE, FitMode::Cover, FitAlign::new(0.5, 0.0));
    assert_eq!(top, m(1.0, 1.0, 0.0, 0.0));
    assert!(top.f.is_sign_positive());
}

#[test]
fn center_is_the_default_alignment() {
    assert_eq!(FitAlign::default(), FitAlign::CENTER);
    assert_eq!(FitAlign::new(0.5, 0.5), FitAlign::CENTER);
}
