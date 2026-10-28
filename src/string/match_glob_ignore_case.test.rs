// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn ignores_case_on_both_sides() {
    assert!(match_glob_ignore_case("screen *", "Screen 2"));
    assert!(match_glob_ignore_case("SCREEN *", "screen 2"));
    assert!(match_glob_ignore_case("É*", "école"));
}

#[test]
fn still_matches_in_full() {
    assert!(!match_glob_ignore_case("screen", "Screen 2"));
    assert!(!match_glob_ignore_case("screen *", "Home"));
    assert!(match_glob_ignore_case("frame-?", "FRAME-1"));
}
