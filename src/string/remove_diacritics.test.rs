// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn removes_common_latin_accents() {
    assert_eq!(remove_diacritics("café"), "cafe");
    assert_eq!(remove_diacritics("naïve"), "naive");
    assert_eq!(remove_diacritics("ÉCOLE"), "ECOLE");
}

#[test]
fn text_without_diacritics_is_unchanged() {
    assert_eq!(remove_diacritics("hello"), "hello");
}

#[test]
fn empty_input_is_empty() {
    assert_eq!(remove_diacritics(""), "");
}

#[test]
fn a_script_with_no_latin_diacritics_is_unchanged() {
    assert_eq!(remove_diacritics("こんにちは"), "こんにちは");
}
