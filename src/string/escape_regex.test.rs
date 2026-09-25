// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn plain_text_is_unchanged() {
    assert_eq!(escape_regex("plain text"), "plain text");
}

#[test]
fn escapes_every_metacharacter() {
    assert_eq!(
        escape_regex(".*+?^$(){}|\\"),
        "\\.\\*\\+\\?\\^\\$\\(\\)\\{\\}\\|\\\\"
    );
}

#[test]
fn empty_input_is_empty() {
    assert_eq!(escape_regex(""), "");
}

#[test]
fn mixes_plain_and_special_characters() {
    assert_eq!(escape_regex("1 + 1 = 2?"), "1 \\+ 1 = 2\\?");
}
