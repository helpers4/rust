// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn converts_camel_case() {
    assert_eq!(constant_case("helloWorld"), "HELLO_WORLD");
}

#[test]
fn converts_separated_words() {
    assert_eq!(constant_case("max retries"), "MAX_RETRIES");
    assert_eq!(constant_case("kebab-case-name"), "KEBAB_CASE_NAME");
}

#[test]
fn keeps_acronyms_together() {
    assert_eq!(constant_case("parseHTTPResponse"), "PARSE_HTTP_RESPONSE");
}

#[test]
fn empty_and_separator_only_input_give_empty() {
    assert_eq!(constant_case(""), "");
    assert_eq!(constant_case(" - _ "), "");
}
