// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn value(line: &str) -> Option<String> {
    parse_line(line).map(|(_, v)| v)
}

#[test]
fn key_validation() {
    for good in ["A", "_a1", "MY_VAR2"] {
        assert!(is_valid_key(good), "{good}");
    }
    for bad in ["", "1A", "a-b", "a b", "é"] {
        assert!(!is_valid_key(bad), "{bad}");
    }
}

#[test]
fn ignores_blank_comment_and_malformed_lines() {
    assert_eq!(parse_line(""), None);
    assert_eq!(parse_line("   "), None);
    assert_eq!(parse_line("# A=1"), None);
    assert_eq!(parse_line("no equals sign"), None);
    assert_eq!(parse_line("1BAD=x"), None);
    assert_eq!(parse_line("=x"), None);
}

#[test]
fn parses_plain_assignments() {
    assert_eq!(parse_line("KEY=value"), Some(("KEY", "value".to_string())));
    assert_eq!(
        parse_line("  KEY = value  "),
        Some(("KEY", "value".to_string()))
    );
    assert_eq!(
        parse_line("export KEY=value"),
        Some(("KEY", "value".to_string()))
    );
    assert_eq!(value("KEY="), Some(String::new()));
    assert_eq!(value("KEY=a=b"), Some("a=b".to_string()));
}

#[test]
fn double_quoted_values_process_escapes() {
    assert_eq!(value(r#"K="a b""#).as_deref(), Some("a b"));
    assert_eq!(
        value(r#"K="l1\nl2\r\t\"q\"\\""#).as_deref(),
        Some("l1\nl2\r\t\"q\"\\")
    );
    assert_eq!(value(r#"K="a\qb""#).as_deref(), Some("a\\qb"));
}

#[test]
fn text_after_the_closing_quote_is_ignored() {
    assert_eq!(value(r#"K="a" # note"#).as_deref(), Some("a"));
}

#[test]
fn unterminated_double_quote_is_kept_literally() {
    assert_eq!(value(r#"K="abc"#).as_deref(), Some("\"abc"));
    assert_eq!(value("K=\"abc\\").as_deref(), Some("\"abc\\"));
}

#[test]
fn single_quoted_values_are_literal() {
    assert_eq!(value(r"K='a\nb # c'").as_deref(), Some(r"a\nb # c"));
    assert_eq!(value("K='abc").as_deref(), Some("'abc"));
}

#[test]
fn bare_values_end_at_a_hash_preceded_by_whitespace() {
    assert_eq!(value("K=abc # note").as_deref(), Some("abc"));
    assert_eq!(value("K=abc\t# note").as_deref(), Some("abc"));
    assert_eq!(value("K=a#b").as_deref(), Some("a#b"));
}
