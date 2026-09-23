// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn accepts_conventional_commits() {
    assert!(is_valid("fix(parser): handle empty input"));
    assert!(is_valid("feat!: a\n\nbody\n\nRefs #1"));
}

#[test]
fn rejects_everything_else() {
    assert!(!is_valid("fixed some stuff"));
    assert!(!is_valid(""));
    assert!(!is_valid("feat: a\nno blank line"));
}
