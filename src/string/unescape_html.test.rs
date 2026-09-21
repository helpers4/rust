// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn decodes_the_named_entities() {
    assert_eq!(unescape_html("&amp;&lt;&gt;&quot;&apos;"), "&<>\"'");
}

#[test]
fn decodes_decimal_and_hexadecimal_references() {
    assert_eq!(unescape_html("&#65;&#x42;&#X43;&#39;"), "ABC'");
    assert_eq!(unescape_html("&#128512;"), "\u{1f600}");
}

#[test]
fn decodes_in_a_single_pass() {
    assert_eq!(unescape_html("&amp;lt;"), "&lt;");
    assert_eq!(unescape_html("&amp;#65;"), "&#65;");
}

#[test]
fn leaves_unknown_and_malformed_entities() {
    assert_eq!(unescape_html("&unknown;"), "&unknown;");
    assert_eq!(unescape_html("a & b"), "a & b");
    assert_eq!(unescape_html("&amp"), "&amp");
    assert_eq!(unescape_html("&"), "&");
    assert_eq!(unescape_html("&;"), "&;");
    assert_eq!(unescape_html("&#;"), "&#;");
    assert_eq!(unescape_html("&#x;"), "&#x;");
    assert_eq!(unescape_html("&#+65;"), "&#+65;");
    assert_eq!(unescape_html("&#xZZ;"), "&#xZZ;");
}

#[test]
fn leaves_references_that_are_not_valid_characters() {
    assert_eq!(unescape_html("&#0;"), "&#0;");
    assert_eq!(unescape_html("&#xD800;"), "&#xD800;");
    assert_eq!(unescape_html("&#x110000;"), "&#x110000;");
    assert_eq!(
        unescape_html("&#99999999999999999999;"),
        "&#99999999999999999999;"
    );
}

#[test]
fn does_not_search_for_the_semicolon_past_the_longest_entity() {
    assert_eq!(
        unescape_html("&aaaaaaaaaaaaaaaaaaaa;"),
        "&aaaaaaaaaaaaaaaaaaaa;"
    );
    assert_eq!(unescape_html("&#x0000000041;"), "&#x0000000041;");
}

#[test]
fn keeps_multibyte_text_around_entities() {
    assert_eq!(unescape_html("é&amp;ü&é;"), "é&ü&é;");
}

#[test]
fn borrows_when_there_is_no_ampersand() {
    assert!(matches!(unescape_html("plain"), Cow::Borrowed("plain")));
}
