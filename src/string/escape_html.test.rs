// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn is_borrowed(s: &str) -> bool {
    matches!(escape_html(s), Cow::Borrowed(_))
}

#[test]
fn escapes_all_five_characters() {
    assert_eq!(escape_html("&<>\"'"), "&amp;&lt;&gt;&quot;&#39;");
}

#[test]
fn keeps_other_characters() {
    assert_eq!(escape_html("a <b> é"), "a &lt;b&gt; é");
}

#[test]
fn borrows_when_nothing_to_escape() {
    assert!(is_borrowed("plain é"));
    assert!(!is_borrowed("a&b"));
}
