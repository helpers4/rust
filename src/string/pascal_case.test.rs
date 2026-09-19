// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn converts_common_formats() {
    assert_eq!(pascal_case("hello-world"), "HelloWorld");
    assert_eq!(pascal_case("user_name"), "UserName");
    assert_eq!(pascal_case("camelCase"), "CamelCase");
    assert_eq!(pascal_case("userID"), "UserId");
}

#[test]
fn empty_input_is_empty() {
    assert_eq!(pascal_case(""), "");
}
