// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

/// Undoes `escape`: drops a backslash that comes before an ASCII punctuation character.
fn unescape(text: &str) -> String {
    let mut out = String::new();
    let mut chars = text.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\\' && chars.peek().is_some_and(char::is_ascii_punctuation) {
            out.extend(chars.next());
        } else {
            out.push(c);
        }
    }
    out
}

proptest! {
    #[test]
    fn unescaping_gives_back_the_text(s in "[ -~\\n]{0,40}") {
        prop_assert_eq!(unescape(&escape(&s)), s);
    }

    #[test]
    fn never_shortens_the_text(s in "\\PC{0,40}") {
        prop_assert!(escape(&s).len() >= s.len());
    }
}
