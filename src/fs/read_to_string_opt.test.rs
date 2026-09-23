// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

use crate::fs::scratch::Scratch;

#[test]
fn reads_an_existing_file() {
    let dir = Scratch::new("read-some");
    let file = dir.path().join("a.txt");
    std::fs::write(&file, "hello").unwrap();
    assert_eq!(
        read_to_string_opt(&file).unwrap(),
        Some("hello".to_string())
    );
}

#[test]
fn a_missing_file_is_none() {
    let dir = Scratch::new("read-none");
    assert_eq!(
        read_to_string_opt(dir.path().join("missing.txt")).unwrap(),
        None
    );
}

#[test]
fn other_errors_are_still_errors() {
    let dir = Scratch::new("read-err");
    assert!(read_to_string_opt(dir.path()).is_err()); // a directory
    let file = dir.path().join("bad.bin");
    std::fs::write(&file, [0xff, 0xfe]).unwrap(); // not UTF-8
    assert!(read_to_string_opt(&file).is_err());
}
