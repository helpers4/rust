// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

use crate::fs::scratch::Scratch;

fn touch(path: &Path) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, "x").unwrap();
}

#[test]
fn lists_files_at_every_depth_sorted() {
    let dir = Scratch::new("walk-tree");
    let root = dir.path();
    touch(&root.join("b.txt"));
    touch(&root.join("a").join("c.txt"));
    touch(&root.join("a").join("deep").join("d.txt"));
    assert_eq!(
        walk(root).unwrap(),
        vec![
            root.join("a").join("c.txt"),
            root.join("a").join("deep").join("d.txt"),
            root.join("b.txt"),
        ]
    );
}

#[test]
fn empty_directories_contribute_nothing() {
    let dir = Scratch::new("walk-empty");
    std::fs::create_dir(dir.path().join("empty")).unwrap();
    assert!(walk(dir.path()).unwrap().is_empty());
}

#[test]
fn a_missing_or_non_directory_path_is_an_error() {
    let dir = Scratch::new("walk-err");
    assert!(walk(dir.path().join("missing")).is_err());
    let file = dir.path().join("f.txt");
    touch(&file);
    assert!(walk(&file).is_err());
}

#[cfg(unix)]
#[test]
fn symbolic_links_are_listed_and_not_followed() {
    let dir = Scratch::new("walk-link");
    let root = dir.path();
    touch(&root.join("real").join("f.txt"));
    std::os::unix::fs::symlink(root, root.join("loop")).unwrap();
    assert_eq!(
        walk(root).unwrap(),
        vec![root.join("loop"), root.join("real").join("f.txt")]
    );
}
