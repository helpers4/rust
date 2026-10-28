// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

const IDENTITY: Matrix = Matrix::IDENTITY;

#[test]
fn moves_a_path_writing_it_absolute() {
    assert_eq!(
        transform_path("M0 0 h10 v10 z", Matrix::translate(5.0, 5.0), 6),
        Ok("M5 5 L15 5 L15 15 Z".to_string())
    );
}

#[test]
fn transforms_with_a_rotation_and_a_scale() {
    let quarter = Matrix::new(0.0, 2.0, -2.0, 0.0, 0.0, 0.0);
    assert_eq!(
        transform_path("M1 0 L0 1", quarter, 6),
        Ok("M0 2 L-2 0".to_string())
    );
    assert_eq!(
        transform_path("M1 0 L0 1", Matrix::rotate(std::f64::consts::FRAC_PI_2), 3),
        Ok("M0 1 L-1 0".to_string())
    );
    assert_eq!(
        transform_path("M1 1", Matrix::scale(2.0, 3.0), 6),
        Ok("M2 3".to_string())
    );
}

#[test]
fn transforms_curve_control_points_too() {
    assert_eq!(
        transform_path("M0 0 C1 1 2 2 3 3 Q1 1 2 2", Matrix::translate(1.0, 0.0), 6),
        Ok("M1 0 C2 1 3 2 4 3 Q2 1 3 2".to_string())
    );
}

#[test]
fn agrees_with_apply_on_every_end_point() {
    let m = Matrix::new(1.5, 0.2, -0.3, 0.8, 7.0, -2.0);
    let moved = parse_path(&transform_path("M1 2 L3 4", m, 9).unwrap()).unwrap();
    for (command, (x, y)) in moved.iter().zip([(1.0, 2.0), (3.0, 4.0)]) {
        let expected = m.apply(Point::new(x, y));
        assert!((command.values[0] - expected.x).abs() < 1e-9);
        assert!((command.values[1] - expected.y).abs() < 1e-9);
    }
}

#[test]
fn leaves_a_path_alone_under_the_identity() {
    assert_eq!(
        transform_path("M1 1 L2 2", IDENTITY, 6),
        Ok("M1 1 L2 2".to_string())
    );
}

#[test]
fn rejects_invalid_path_data() {
    assert_eq!(
        transform_path("L1 1", IDENTITY, 6),
        Err(ParsePathError::MissingMove)
    );
}
