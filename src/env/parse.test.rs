// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn keeps_file_order_and_skips_non_assignments() {
    let content = "# c\nA=1\n\nnot an assignment\nB=two words # note\nexport C='x'\n";
    assert_eq!(
        parse(content),
        [
            ("A".to_string(), "1".to_string()),
            ("B".to_string(), "two words".to_string()),
            ("C".to_string(), "x".to_string()),
        ]
    );
}

#[test]
fn duplicate_keys_are_all_returned() {
    assert_eq!(parse("A=1\nA=2").len(), 2);
}

#[test]
fn handles_crlf_line_endings() {
    assert_eq!(
        parse("A=1\r\nB=2\r\n"),
        [
            ("A".to_string(), "1".to_string()),
            ("B".to_string(), "2".to_string())
        ]
    );
}

#[test]
fn empty_content_has_no_variables() {
    assert!(parse("").is_empty());
}
