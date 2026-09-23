// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn wraps_plain_text_in_single_backticks() {
    assert_eq!(inline_code("cargo test"), "`cargo test`");
    assert_eq!(inline_code(""), "``");
}

#[test]
fn uses_a_longer_fence_than_any_run_inside() {
    assert_eq!(inline_code("a`b"), "``a`b``");
    assert_eq!(inline_code("a``b"), "```a``b```");
    assert_eq!(inline_code("a`b``c"), "```a`b``c```");
}

#[test]
fn pads_text_that_starts_or_ends_with_a_backtick() {
    assert_eq!(inline_code("`tick"), "`` `tick ``");
    assert_eq!(inline_code("tick`"), "`` tick` ``");
    assert_eq!(inline_code("`"), "`` ` ``");
}

#[test]
fn pads_text_that_starts_and_ends_with_a_space_so_they_survive() {
    assert_eq!(inline_code(" a "), "`  a  `");
    assert_eq!(inline_code(" a"), "` a`");
    assert_eq!(inline_code("a "), "`a `");
}

#[test]
fn a_text_of_only_spaces_needs_no_padding() {
    assert_eq!(inline_code("  "), "`  `");
}
