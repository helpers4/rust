// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn italic_and_oblique_names_are_italic() {
    assert!(is_italic("Italic"));
    assert!(is_italic("Semi Bold Italic"));
    assert!(is_italic("Light Oblique"));
    assert!(is_italic("BOLDITALIC"));
}

#[test]
fn other_names_are_not() {
    assert!(!is_italic("Bold"));
    assert!(!is_italic("Regular"));
    assert!(!is_italic(""));
}
