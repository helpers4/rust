// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn w(s: &str) -> Vec<String> {
    words(s)
}

#[test]
fn empty_and_separator_only_inputs_have_no_words() {
    assert!(w("").is_empty());
    assert!(w(" -_ ").is_empty());
}

#[test]
fn splits_on_non_alphanumeric() {
    assert_eq!(
        w("hello-world_foo bar.baz"),
        ["hello", "world", "foo", "bar", "baz"]
    );
}

#[test]
fn splits_lower_to_upper_and_digit_to_upper() {
    assert_eq!(w("userName"), ["user", "name"]);
    assert_eq!(w("v2Beta"), ["v2", "beta"]);
}

#[test]
fn keeps_acronym_runs_together_until_the_last_capital() {
    assert_eq!(w("HTMLParser"), ["html", "parser"]);
    assert_eq!(w("userID"), ["user", "id"]);
    assert_eq!(w("ABC"), ["abc"]);
}

#[test]
fn does_not_split_letters_from_trailing_digits() {
    assert_eq!(w("version2"), ["version2"]);
}

#[test]
fn handles_caseless_scripts() {
    assert_eq!(w("日本語 テスト"), ["日本語", "テスト"]);
}
