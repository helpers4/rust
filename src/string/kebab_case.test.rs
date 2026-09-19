// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn converts_common_formats() {
    assert_eq!(kebab_case("helloWorld"), "hello-world");
    assert_eq!(kebab_case("snake_case_input"), "snake-case-input");
    assert_eq!(kebab_case("Hello World"), "hello-world");
}

#[test]
fn empty_input_is_empty() {
    assert_eq!(kebab_case(""), "");
}
