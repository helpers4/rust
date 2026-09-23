// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

use crate::fs::scratch::Scratch;

fn names(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = std::fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

#[test]
fn creates_a_new_file() {
    let dir = Scratch::new("atomic-new");
    let file = dir.path().join("a.txt");
    write_atomic(&file, "hello").unwrap();
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "hello");
}

#[test]
fn replaces_an_existing_file_and_leaves_nothing_behind() {
    let dir = Scratch::new("atomic-replace");
    let file = dir.path().join("a.txt");
    write_atomic(&file, "first").unwrap();
    write_atomic(&file, b"second").unwrap();
    assert_eq!(std::fs::read_to_string(&file).unwrap(), "second");
    assert_eq!(names(dir.path()), vec!["a.txt".to_string()]);
}

#[test]
fn a_missing_parent_directory_is_an_error() {
    let dir = Scratch::new("atomic-noparent");
    assert!(write_atomic(dir.path().join("missing").join("a.txt"), "x").is_err());
}

#[test]
fn a_path_without_a_file_name_is_an_error() {
    let error = write_atomic("..", "x").unwrap_err();
    assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
}

#[test]
fn a_failed_rename_cleans_up_and_keeps_the_target() {
    let dir = Scratch::new("atomic-rename-fails");
    let target = dir.path().join("target");
    std::fs::create_dir(&target).unwrap();
    assert!(write_atomic(&target, "x").is_err());
    assert_eq!(names(dir.path()), vec!["target".to_string()]);
    assert!(target.is_dir());
}
