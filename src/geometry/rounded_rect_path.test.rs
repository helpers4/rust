// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

const BOX: Rect = Rect::new(0.0, 0.0, 10.0, 10.0);

#[test]
fn draws_a_plain_rectangle_without_radius() {
    assert_eq!(
        rounded_rect_path(BOX, [0.0; 4]),
        "M0 0 L10 0 L10 10 L0 10 L0 0 Z"
    );
}

#[test]
fn rounds_the_four_corners_with_one_radius() {
    assert_eq!(
        rounded_rect_path(Rect::new(1.0, 2.0, 10.0, 20.0), [2.0; 4]),
        "M3 2 L9 2 A2 2 0 0 1 11 4 L11 20 A2 2 0 0 1 9 22 L3 22 A2 2 0 0 1 1 20 L1 4 A2 2 0 0 1 3 2 Z"
    );
}

#[test]
fn takes_a_radius_per_corner_in_css_order() {
    assert_eq!(
        rounded_rect_path(BOX, [5.0, 0.0, 0.0, 0.0]),
        "M5 0 L10 0 L10 10 L0 10 L0 5 A5 5 0 0 1 5 0 Z"
    );
    assert_eq!(
        rounded_rect_path(BOX, [0.0, 4.0, 0.0, 0.0]),
        "M0 0 L6 0 A4 4 0 0 1 10 4 L10 10 L0 10 L0 0 Z"
    );
    assert_eq!(
        rounded_rect_path(BOX, [0.0, 0.0, 3.0, 0.0]),
        "M0 0 L10 0 L10 7 A3 3 0 0 1 7 10 L0 10 L0 0 Z"
    );
    assert_eq!(
        rounded_rect_path(BOX, [0.0, 0.0, 0.0, 2.0]),
        "M0 0 L10 0 L10 10 L2 10 A2 2 0 0 1 0 8 L0 0 Z"
    );
}

#[test]
fn clamps_radii_to_half_the_smaller_side() {
    assert_eq!(
        rounded_rect_path(BOX, [100.0; 4]),
        "M5 0 L5 0 A5 5 0 0 1 10 5 L10 5 A5 5 0 0 1 5 10 L5 10 A5 5 0 0 1 0 5 L0 5 A5 5 0 0 1 5 0 Z"
    );
    assert!(rounded_rect_path(Rect::new(0.0, 0.0, 20.0, 4.0), [9.0; 4]).contains("A2 2"));
}

#[test]
fn counts_a_negative_radius_as_zero() {
    assert_eq!(
        rounded_rect_path(BOX, [-3.0; 4]),
        rounded_rect_path(BOX, [0.0; 4])
    );
}
