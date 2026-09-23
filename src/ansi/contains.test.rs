// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn detects_an_escape_character() {
    assert!(contains("\u{1b}[31mred"));
    assert!(contains("a\u{1b}"));
}

#[test]
fn plain_text_has_none() {
    assert!(!contains("plain"));
    assert!(!contains(""));
    assert!(!contains("[31m"));
}
