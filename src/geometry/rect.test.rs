// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn new_sets_the_fields() {
    assert_eq!(
        Rect::new(1.0, 2.0, 3.0, 4.0),
        Rect {
            x: 1.0,
            y: 2.0,
            width: 3.0,
            height: 4.0
        }
    );
}

#[test]
fn right_and_bottom_add_the_size() {
    let r = Rect::new(-1.0, -2.0, 3.0, 5.0);
    assert_eq!((r.right(), r.bottom()), (2.0, 3.0));
}

#[test]
fn default_is_empty_at_the_origin() {
    assert_eq!(Rect::default(), Rect::new(0.0, 0.0, 0.0, 0.0));
}
