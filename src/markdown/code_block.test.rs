// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn fences_code_with_its_language() {
    assert_eq!(
        code_block("let x = 1;", "rust"),
        "```rust\nlet x = 1;\n```\n"
    );
}

#[test]
fn the_language_is_optional() {
    assert_eq!(code_block("a", ""), "```\na\n```\n");
}

#[test]
fn keeps_multiline_code_and_does_not_double_the_trailing_newline() {
    assert_eq!(code_block("a\nb", "sh"), "```sh\na\nb\n```\n");
    assert_eq!(code_block("a\nb\n", "sh"), "```sh\na\nb\n```\n");
    assert_eq!(code_block("a\n\n", "sh"), "```sh\na\n\n```\n");
}

#[test]
fn uses_a_longer_fence_when_the_code_contains_one() {
    assert_eq!(
        code_block("```\nnested\n```", ""),
        "````\n```\nnested\n```\n````\n"
    );
    assert_eq!(code_block("a `` b", ""), "```\na `` b\n```\n");
    assert_eq!(code_block("`````", ""), "``````\n`````\n``````\n");
}

#[test]
fn empty_code_gives_an_empty_block() {
    assert_eq!(code_block("", "rust"), "```rust\n```\n");
}

#[test]
fn sanitizes_the_language() {
    assert_eq!(code_block("a", "ru st`"), "```rust\na\n```\n");
}
