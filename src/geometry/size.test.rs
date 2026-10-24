// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn new_sets_the_fields_and_default_is_empty() {
    assert_eq!(
        Size::new(3.0, 4.0),
        Size {
            width: 3.0,
            height: 4.0
        }
    );
    assert_eq!(Size::default(), Size::new(0.0, 0.0));
}
