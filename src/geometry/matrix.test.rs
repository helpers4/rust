// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn scale(sx: f64, sy: f64) -> Matrix {
    Matrix::new(sx, 0.0, 0.0, sy, 0.0, 0.0)
}

fn translate(tx: f64, ty: f64) -> Matrix {
    Matrix::new(1.0, 0.0, 0.0, 1.0, tx, ty)
}

fn rotate(radians: f64) -> Matrix {
    let (sin, cos) = radians.sin_cos();
    Matrix::new(cos, sin, -sin, cos, 0.0, 0.0)
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

#[test]
fn identity_is_the_default_and_changes_nothing() {
    assert_eq!(Matrix::default(), Matrix::IDENTITY);
    assert_eq!(
        Matrix::IDENTITY.apply(Point::new(3.0, 4.0)),
        Point::new(3.0, 4.0)
    );
}

#[test]
fn apply_follows_the_css_layout() {
    assert_eq!(
        Matrix::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0).apply(Point::new(1.0, 1.0)),
        Point::new(9.0, 12.0)
    );
}

#[test]
fn multiply_applies_the_argument_first() {
    let p = Point::new(1.0, 1.0);
    assert_eq!(
        translate(10.0, 0.0).multiply(scale(2.0, 2.0)).apply(p),
        Point::new(12.0, 2.0)
    );
    assert_eq!(
        scale(2.0, 2.0).multiply(translate(10.0, 0.0)).apply(p),
        Point::new(22.0, 2.0)
    );
}

#[test]
fn multiply_by_identity_changes_nothing() {
    let m = Matrix::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
    assert_eq!(m.multiply(Matrix::IDENTITY), m);
    assert_eq!(Matrix::IDENTITY.multiply(m), m);
}

#[test]
fn invert_undoes_a_translation_without_negative_zeros() {
    let inverse = translate(10.0, 5.0).invert();
    assert_eq!(inverse, Some(translate(-10.0, -5.0)));
    assert!(inverse.is_some_and(|m| m.b.is_sign_positive() && m.c.is_sign_positive()));
}

#[test]
fn invert_undoes_a_rotation_and_a_scale() {
    let m = translate(3.0, 4.0)
        .multiply(rotate(0.7))
        .multiply(scale(2.0, 3.0));
    let p = Point::new(-5.0, 8.0);
    let back = m.invert().map(|inv| inv.apply(m.apply(p)));
    assert!(back.is_some_and(|q| close(q.x, p.x) && close(q.y, p.y)));
}

#[test]
fn invert_refuses_a_flat_or_non_finite_matrix() {
    assert_eq!(scale(0.0, 1.0).invert(), None);
    assert_eq!(Matrix::new(1.0, 2.0, 2.0, 4.0, 0.0, 0.0).invert(), None);
    assert_eq!(translate(f64::NAN, 0.0).invert(), None);
}

#[test]
fn invert_writes_no_negative_zero_in_any_entry() {
    // Each matrix makes one entry of the raw inverse -0.0 (found by exhaustive search over
    // -1, -0, 0 and 1).
    let cases = [
        Matrix::new(-1.0, -1.0, -1.0, 0.0, -1.0, -1.0),
        Matrix::new(-1.0, -0.0, -1.0, 1.0, -1.0, -1.0),
        Matrix::new(-1.0, -1.0, -0.0, 1.0, -1.0, -1.0),
        Matrix::new(-0.0, -1.0, 1.0, -1.0, -1.0, -1.0),
        Matrix::new(-1.0, -1.0, -1.0, -0.0, -1.0, -0.0),
        Matrix::new(-1.0, -1.0, -1.0, -0.0, -1.0, -1.0),
    ];
    for m in cases {
        let inverse = m.invert();
        assert!(inverse.is_some());
        for entry in inverse.iter().flat_map(|i| [i.a, i.b, i.c, i.d, i.e, i.f]) {
            assert!(
                entry != 0.0 || entry.is_sign_positive(),
                "{m:?} gives -0.0 in {inverse:?}"
            );
        }
    }
}

#[test]
fn invert_has_no_tolerance() {
    assert_eq!(scale(1e-9, 1e-9).invert(), Some(scale(1e9, 1e9)));
}

#[test]
fn bounding_box_of_a_scaled_rectangle() {
    assert_eq!(
        scale(2.0, 2.0).bounding_box(Rect::new(1.0, 1.0, 2.0, 3.0)),
        Rect::new(2.0, 2.0, 4.0, 6.0)
    );
}

#[test]
fn bounding_box_keeps_a_positive_size_through_a_flip() {
    assert_eq!(
        scale(-1.0, -1.0).bounding_box(Rect::new(1.0, 1.0, 2.0, 3.0)),
        Rect::new(-3.0, -4.0, 2.0, 3.0)
    );
}

#[test]
fn bounding_box_grows_around_a_rotation() {
    let b = rotate(std::f64::consts::FRAC_PI_4).bounding_box(Rect::new(0.0, 0.0, 1.0, 1.0));
    assert!(close(b.x, -std::f64::consts::FRAC_1_SQRT_2));
    assert!(close(b.y, 0.0));
    assert!(close(b.width, std::f64::consts::SQRT_2));
    assert!(close(b.height, std::f64::consts::SQRT_2));
}

#[test]
fn the_constructors_match_the_hand_written_matrices() {
    assert_eq!(Matrix::translate(10.0, -5.0), translate(10.0, -5.0));
    assert_eq!(Matrix::scale(2.0, -1.0), scale(2.0, -1.0));
    let (sin, cos) = 0.7f64.sin_cos();
    assert_eq!(
        Matrix::rotate(0.7),
        Matrix::new(cos, sin, -sin, cos, 0.0, 0.0)
    );
}

#[test]
fn rotate_turns_clockwise_with_y_down() {
    let p = Matrix::rotate(std::f64::consts::FRAC_PI_2).apply(Point::new(1.0, 0.0));
    assert!(close(p.x, 0.0) && close(p.y, 1.0));
}

#[test]
fn to_css_writes_the_six_entries_in_order_without_negative_zero() {
    assert_eq!(
        Matrix::new(2.0, 0.0, 0.0, 2.0, 10.0, -5.0).to_css(),
        "matrix(2, 0, 0, 2, 10, -5)"
    );
    assert_eq!(
        Matrix::new(0.5, -0.0, -0.0, 0.5, -0.0, 1.25).to_css(),
        "matrix(0.5, 0, 0, 0.5, 0, 1.25)"
    );
    assert_eq!(Matrix::IDENTITY.to_css(), "matrix(1, 0, 0, 1, 0, 0)");
}

#[test]
fn rect_to_rect_translates_and_scales_one_rectangle_onto_another() {
    let from = Rect::new(0.0, 0.0, 10.0, 10.0);
    let to = Rect::new(5.0, 5.0, 20.0, 10.0);
    let m = Matrix::rect_to_rect(from, to);
    assert_eq!(m, Matrix::new(2.0, 0.0, 0.0, 1.0, 5.0, 5.0));
    assert_eq!(m.bounding_box(from), to);
}

#[test]
fn rect_to_rect_is_the_identity_from_a_rectangle_to_itself() {
    let r = Rect::new(3.0, 4.0, 5.0, 6.0);
    assert_eq!(Matrix::rect_to_rect(r, r), Matrix::IDENTITY);
}

#[test]
fn rect_to_rect_only_translates_an_axis_without_extent() {
    assert_eq!(
        Matrix::rect_to_rect(Rect::new(1.0, 1.0, 0.0, 2.0), Rect::new(4.0, 3.0, 9.0, 4.0)),
        Matrix::new(1.0, 0.0, 0.0, 2.0, 3.0, 1.0)
    );
    assert_eq!(
        Matrix::rect_to_rect(Rect::new(1.0, 1.0, 2.0, 0.0), Rect::new(4.0, 3.0, 4.0, 9.0)),
        Matrix::new(2.0, 0.0, 0.0, 1.0, 2.0, 2.0)
    );
}
