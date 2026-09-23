// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn writes_a_plain_link() {
    assert_eq!(
        link("docs", "https://helpers4.dev/rust/"),
        "[docs](https://helpers4.dev/rust/)"
    );
}

#[test]
fn escapes_brackets_and_backslashes_in_the_text() {
    assert_eq!(link("[1]", "u"), "[\\[1\\]](u)");
    assert_eq!(link("a\\b", "u"), "[a\\\\b](u)");
}

#[test]
fn encodes_the_characters_that_would_end_the_destination() {
    assert_eq!(link("t", "a b"), "[t](a%20b)");
    assert_eq!(link("t", "a(b)"), "[t](a%28b%29)");
    assert_eq!(link("t", "a<b>"), "[t](a%3Cb%3E)");
    assert_eq!(link("t", "a\nb"), "[t](a%0Ab)");
}

#[test]
fn leaves_the_rest_of_the_url_alone() {
    assert_eq!(
        link("t", "https://x.org/p?q=1&r=%20#f"),
        "[t](https://x.org/p?q=1&r=%20#f)"
    );
}

#[test]
fn empty_parts_are_allowed() {
    assert_eq!(link("", ""), "[]()");
}
