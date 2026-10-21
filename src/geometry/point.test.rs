// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn new_sets_the_coordinates() {
    assert_eq!(Point::new(1.5, -2.0), Point { x: 1.5, y: -2.0 });
}

#[test]
fn default_is_the_origin() {
    assert_eq!(Point::default(), Point::new(0.0, 0.0));
}
