// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn ignores_escape_sequences() {
    assert_eq!(visible_len("\u{1b}[1;31merror\u{1b}[0m"), 5);
}

#[test]
fn counts_plain_text_and_empty_text() {
    assert_eq!(visible_len("plain"), 5);
    assert_eq!(visible_len(""), 0);
}

#[test]
fn counts_characters_not_bytes() {
    assert_eq!(visible_len("\u{1b}[1mé日\u{1b}[0m"), 2);
}
