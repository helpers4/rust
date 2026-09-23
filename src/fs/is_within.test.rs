// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn a_path_below_the_base_is_inside() {
    assert!(is_within(Path::new("/srv"), Path::new("a/b.txt")));
    assert!(is_within(Path::new("/srv"), Path::new("a/../b")));
}

#[test]
fn the_base_itself_is_inside() {
    assert!(is_within(Path::new("/srv"), Path::new("")));
    assert!(is_within(Path::new("/srv"), Path::new(".")));
    assert!(is_within(Path::new("/srv"), Path::new("a/..")));
}

#[test]
fn a_path_that_climbs_out_is_outside() {
    assert!(!is_within(Path::new("/srv"), Path::new("../etc")));
    assert!(!is_within(
        Path::new("/srv/uploads"),
        Path::new("a/../../b")
    ));
}

#[test]
fn a_sibling_with_the_same_prefix_is_outside() {
    assert!(!is_within(Path::new("/srv/up"), Path::new("../uploads/x")));
}

#[test]
fn an_absolute_path_must_lie_under_the_base() {
    assert!(is_within(Path::new("/srv"), Path::new("/srv/a")));
    assert!(!is_within(Path::new("/srv"), Path::new("/etc/passwd")));
}

#[test]
fn a_relative_base_works_too() {
    assert!(is_within(Path::new("."), Path::new("a/b")));
    assert!(!is_within(Path::new("."), Path::new("../a")));
    assert!(is_within(Path::new("data"), Path::new("x")));
    assert!(!is_within(Path::new("data"), Path::new("../x")));
}
