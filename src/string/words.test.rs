// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn empty_and_separator_only_inputs_have_no_words() {
    assert!(words("").is_empty());
    assert!(words(" -_ ").is_empty());
}

#[test]
fn splits_on_whitespace_and_punctuation() {
    assert_eq!(words("hello world"), vec!["hello", "world"]);
    assert_eq!(
        words("hello-world_foo.bar"),
        vec!["hello", "world", "foo", "bar"]
    );
}

#[test]
fn preserves_original_casing() {
    assert_eq!(words("camelCaseString"), vec!["camel", "Case", "String"]);
    assert_eq!(words("PascalCase"), vec!["Pascal", "Case"]);
    assert_eq!(words("SCREAMING_SNAKE"), vec!["SCREAMING", "SNAKE"]);
    assert_eq!(words("HTMLParser"), vec!["HTML", "Parser"]);
}

#[test]
fn letters_and_digits_without_a_case_change_stay_one_word() {
    assert_eq!(words("foo123bar"), vec!["foo123bar"]);
}
