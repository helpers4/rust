// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn cleans_dots_and_parent_dirs() {
    assert_eq!(normalize(Path::new("a/./b/../c")), PathBuf::from("a/c"));
}

#[test]
fn keeps_leading_parent_dirs_of_a_relative_path() {
    assert_eq!(normalize(Path::new("../a")), PathBuf::from("../a"));
}

#[test]
fn drops_parent_dirs_above_the_root() {
    assert_eq!(normalize(Path::new("/../a")), PathBuf::from("/a"));
}

#[test]
fn a_path_that_cleans_to_nothing_is_a_dot() {
    assert_eq!(normalize(Path::new("a/..")), PathBuf::from("."));
    assert_eq!(normalize(Path::new("")), PathBuf::from("."));
    assert_eq!(normalize(Path::new(".")), PathBuf::from("."));
}

#[test]
fn a_clean_path_is_unchanged() {
    assert_eq!(
        normalize(Path::new("a/b/c.txt")),
        PathBuf::from("a/b/c.txt")
    );
}
