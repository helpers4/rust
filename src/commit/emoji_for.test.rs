// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn every_standard_type_has_its_emoji() {
    assert_eq!(emoji_for("feat"), Some("\u{2728}"));
    assert_eq!(emoji_for("fix"), Some("\u{1f41b}"));
    assert_eq!(emoji_for("docs"), Some("\u{1f4dd}"));
    assert_eq!(emoji_for("refactor"), Some("\u{267b}\u{fe0f}"));
    assert_eq!(emoji_for("test"), Some("\u{2705}"));
    assert_eq!(emoji_for("chore"), Some("\u{1f527}"));
    assert_eq!(emoji_for("perf"), Some("\u{26a1}\u{fe0f}"));
    assert_eq!(emoji_for("style"), Some("\u{1f484}"));
    assert_eq!(emoji_for("ci"), Some("\u{1f477}"));
    assert_eq!(emoji_for("build"), Some("\u{1f4e6}\u{fe0f}"));
    assert_eq!(emoji_for("revert"), Some("\u{23ea}\u{fe0f}"));
}

#[test]
fn ignores_case() {
    assert_eq!(emoji_for("FEAT"), emoji_for("feat"));
}

#[test]
fn an_unknown_type_has_no_emoji() {
    assert_eq!(emoji_for("wip"), None);
    assert_eq!(emoji_for(""), None);
}
