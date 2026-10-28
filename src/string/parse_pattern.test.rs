// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn regexp(source: &str, flags: &str) -> Pattern {
    Pattern::RegExp {
        source: source.to_string(),
        flags: flags.to_string(),
    }
}

#[test]
fn reads_source_and_flags_as_a_regular_expression() {
    assert_eq!(parse_pattern("/^SF Pro/i"), regexp("^SF Pro", "i"));
    assert_eq!(parse_pattern("/a.c/"), regexp("a.c", ""));
    assert_eq!(parse_pattern("/a/b/gi"), regexp("a/b", "gi"));
}

#[test]
fn reads_anything_else_as_a_glob() {
    assert_eq!(
        parse_pattern("SF Pro*"),
        Pattern::Glob("SF Pro*".to_string())
    );
    assert_eq!(parse_pattern(""), Pattern::Glob(String::new()));
    assert_eq!(
        parse_pattern("/src/*.ts"),
        Pattern::Glob("/src/*.ts".to_string())
    );
    assert_eq!(parse_pattern("/a/x"), Pattern::Glob("/a/x".to_string()));
}

#[test]
fn needs_a_source_between_the_slashes() {
    assert_eq!(parse_pattern("//"), Pattern::Glob("//".to_string()));
    assert_eq!(parse_pattern("/"), Pattern::Glob("/".to_string()));
    assert_eq!(parse_pattern("//i"), Pattern::Glob("//i".to_string()));
}
