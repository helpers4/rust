// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn puts_exactly_one_slash_between_the_parts() {
    assert_eq!(join_path("a", "b"), "a/b");
    assert_eq!(join_path("a/", "b"), "a/b");
    assert_eq!(join_path("a", "/b"), "a/b");
    assert_eq!(join_path("a/", "/b"), "a/b");
    assert_eq!(join_path("a//", "//b"), "a/b");
}

#[test]
fn works_on_full_urls() {
    assert_eq!(
        join_path("https://api.example.com/v1/", "/users"),
        "https://api.example.com/v1/users"
    );
}

#[test]
fn empty_parts_leave_a_lone_slash() {
    assert_eq!(join_path("", ""), "/");
    assert_eq!(join_path("a", ""), "a/");
    assert_eq!(join_path("", "b"), "/b");
}
