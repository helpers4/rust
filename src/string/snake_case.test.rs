// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn converts_common_formats() {
    assert_eq!(snake_case("helloWorld"), "hello_world");
    assert_eq!(snake_case("kebab-case-input"), "kebab_case_input");
    assert_eq!(snake_case("HTMLParser"), "html_parser");
}

#[test]
fn empty_input_is_empty() {
    assert_eq!(snake_case(""), "");
}
