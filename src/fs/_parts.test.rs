// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use std::path::PathBuf;

fn joined(path: &str) -> PathBuf {
    parts(Path::new(path)).into_iter().collect()
}

#[test]
fn removes_current_dir_components() {
    assert_eq!(joined("a/./b/."), PathBuf::from("a/b"));
}

#[test]
fn folds_parent_dirs_into_the_previous_component() {
    assert_eq!(joined("a/b/../c"), PathBuf::from("a/c"));
    assert_eq!(joined("a/b/../../c"), PathBuf::from("c"));
}

#[test]
fn keeps_parent_dirs_that_have_nothing_to_fold_into() {
    assert_eq!(joined("../a"), PathBuf::from("../a"));
    assert_eq!(joined("a/../../b"), PathBuf::from("../b"));
    assert_eq!(joined("../../a"), PathBuf::from("../../a"));
}

#[test]
fn cannot_go_above_the_root() {
    assert_eq!(joined("/../a"), PathBuf::from("/a"));
    assert_eq!(joined("/a/../../b"), PathBuf::from("/b"));
}

#[test]
fn an_empty_or_dot_path_has_no_parts() {
    assert!(parts(Path::new("")).is_empty());
    assert!(parts(Path::new(".")).is_empty());
    assert!(parts(Path::new("a/..")).is_empty());
}
