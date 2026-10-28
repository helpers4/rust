// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn splits_at_the_first_hyphen() {
    assert_eq!(
        split_postscript("NeutrifPro-SemiBold"),
        ("NeutrifPro", "SemiBold")
    );
    assert_eq!(split_postscript("A-B-C"), ("A", "B-C"));
}

#[test]
fn without_a_hyphen_the_style_is_empty() {
    assert_eq!(split_postscript("Helvetica"), ("Helvetica", ""));
    assert_eq!(split_postscript(""), ("", ""));
}

#[test]
fn keeps_empty_sides() {
    assert_eq!(split_postscript("-Bold"), ("", "Bold"));
    assert_eq!(split_postscript("Name-"), ("Name", ""));
}
