// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn converts_common_formats() {
    assert_eq!(camel_case("hello-world"), "helloWorld");
    assert_eq!(camel_case("user_name"), "userName");
    assert_eq!(camel_case("Hello World"), "helloWorld");
    assert_eq!(camel_case("PascalCase"), "pascalCase");
}

#[test]
fn treats_embedded_acronyms_as_boundaries() {
    assert_eq!(camel_case("userID"), "userId");
}

#[test]
fn empty_or_separator_only_input_is_empty() {
    assert_eq!(camel_case(""), "");
    assert_eq!(camel_case("--"), "");
}

#[test]
fn single_word_is_lowercased() {
    assert_eq!(camel_case("HELLO"), "hello");
}
