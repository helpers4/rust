// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn removes_every_assignment_of_the_key() {
    assert_eq!(remove("A=1\nB=2\nA=3\n", "A"), "B=2\n");
}

#[test]
fn keeps_comments_and_blank_lines() {
    assert_eq!(remove("# A=1\n\nA=1\nB=2", "A"), "# A=1\n\nB=2");
}

#[test]
fn unknown_key_leaves_content_unchanged() {
    assert_eq!(remove("A=1\n", "B"), "A=1\n");
    assert_eq!(remove("", "B"), "");
}
