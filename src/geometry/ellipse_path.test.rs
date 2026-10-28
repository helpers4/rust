// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn draws_the_ellipse_in_its_box_with_two_arcs() {
    assert_eq!(
        ellipse_path(Rect::new(0.0, 0.0, 10.0, 6.0)),
        "M0 3 A5 3 0 0 1 10 3 A5 3 0 0 1 0 3 Z"
    );
}

#[test]
fn draws_a_circle_in_a_square_box_offset() {
    assert_eq!(
        ellipse_path(Rect::new(10.0, 20.0, 4.0, 4.0)),
        "M10 22 A2 2 0 0 1 14 22 A2 2 0 0 1 10 22 Z"
    );
}
