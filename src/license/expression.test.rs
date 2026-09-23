// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn expr(text: &str) -> Expression {
    Expression::parse(text).unwrap()
}

#[test]
fn parses_a_single_license() {
    assert_eq!(expr("MIT").licenses(), ["MIT"]);
    assert_eq!(expr("  GPL-2.0+  ").licenses(), ["GPL-2.0+"]);
    assert_eq!(expr("LicenseRef-Custom").licenses(), ["LicenseRef-Custom"]);
    assert_eq!(
        expr("DocumentRef-x:LicenseRef-y").licenses(),
        ["DocumentRef-x:LicenseRef-y"]
    );
}

#[test]
fn parses_or_and_and() {
    assert_eq!(expr("MIT OR Apache-2.0").licenses(), ["MIT", "Apache-2.0"]);
    assert_eq!(expr("MIT AND Apache-2.0").licenses(), ["MIT", "Apache-2.0"]);
}

#[test]
fn and_binds_tighter_than_or() {
    // MIT OR (Apache-2.0 AND BSD-3-Clause)
    let e = expr("MIT OR Apache-2.0 AND BSD-3-Clause");
    assert!(e.is_satisfied_by(&["MIT"]));
    assert!(!e.is_satisfied_by(&["Apache-2.0"]));
    assert!(e.is_satisfied_by(&["Apache-2.0", "BSD-3-Clause"]));
}

#[test]
fn parentheses_change_the_grouping() {
    let e = expr("(MIT OR Apache-2.0) AND BSD-3-Clause");
    assert!(!e.is_satisfied_by(&["MIT"]));
    assert!(e.is_satisfied_by(&["MIT", "BSD-3-Clause"]));
    assert!(e.is_satisfied_by(&["Apache-2.0", "BSD-3-Clause"]));
}

#[test]
fn nested_operators_of_the_same_kind_are_the_same_expression() {
    assert_eq!(expr("A AND (B AND C)"), expr("A AND B AND C"));
    assert_eq!(expr("(A AND B) AND C"), expr("A AND B AND C"));
    assert_eq!(expr("A OR (B OR C)"), expr("A OR B OR C"));
    assert_eq!(expr("A OR (B / C)"), expr("A OR B OR C"));
    assert_ne!(expr("A AND (B OR C)"), expr("A AND B OR C"));
    assert_eq!(expr("A AND (B AND C)").to_string(), "A AND B AND C");
}

#[test]
fn reads_the_legacy_slash_as_or() {
    let e = expr("MIT/Apache-2.0");
    assert_eq!(e.licenses(), ["MIT", "Apache-2.0"]);
    assert!(e.is_satisfied_by(&["Apache-2.0"]));
    assert_eq!(expr("MIT / Apache-2.0").to_string(), "MIT OR Apache-2.0");
}

#[test]
fn parses_exceptions() {
    let e = expr("Apache-2.0 WITH LLVM-exception");
    assert_eq!(e.licenses(), ["Apache-2.0"]);
    assert!(e.is_satisfied_by(&["Apache-2.0"]));
    assert_eq!(e.to_string(), "Apache-2.0 WITH LLVM-exception");
    assert_eq!(
        expr("MIT OR GPL-2.0-only WITH Classpath-exception-2.0").licenses(),
        ["MIT", "GPL-2.0-only"]
    );
}

#[test]
fn lists_each_license_once_in_order() {
    assert_eq!(
        expr("MIT OR (Apache-2.0 AND MIT) OR ISC").licenses(),
        ["MIT", "Apache-2.0", "ISC"]
    );
}

#[test]
fn a_satisfied_policy_compares_without_regard_to_case() {
    assert!(expr("MIT").is_satisfied_by(&["mit"]));
    assert!(!expr("MIT").is_satisfied_by(&[]));
    assert!(!expr("GPL-2.0+").is_satisfied_by(&["GPL-2.0-or-later"]));
}

#[test]
fn finds_the_identifiers_it_does_not_list() {
    let e =
        expr("MIT OR Fake-1.0 OR LicenseRef-x OR DocumentRef-a:LicenseRef-b OR GPL-2.0+ OR Nope+");
    assert_eq!(e.unknown_ids(), ["Fake-1.0", "Nope+"]);
    assert!(expr("MIT AND Apache-2.0").unknown_ids().is_empty());
}

#[test]
fn displays_with_parentheses_only_where_needed() {
    for text in [
        "MIT",
        "MIT OR Apache-2.0",
        "MIT AND Apache-2.0",
        "MIT OR Apache-2.0 AND BSD-3-Clause",
        "(MIT OR Apache-2.0) AND BSD-3-Clause",
        "MIT AND (Apache-2.0 OR ISC) AND BSD-3-Clause",
    ] {
        assert_eq!(expr(text).to_string(), text);
    }
    assert_eq!(expr("((MIT))").to_string(), "MIT");
    assert_eq!("MIT".parse::<Expression>(), Expression::parse("MIT"));
}

#[test]
fn rejects_an_empty_expression() {
    assert_eq!(Expression::parse(""), Err(ParseExpressionError::Empty));
    assert_eq!(Expression::parse("   "), Err(ParseExpressionError::Empty));
}

#[test]
fn rejects_an_expression_that_ends_too_early() {
    assert_eq!(
        Expression::parse("MIT OR"),
        Err(ParseExpressionError::UnexpectedEnd)
    );
    assert_eq!(
        Expression::parse("MIT AND"),
        Err(ParseExpressionError::UnexpectedEnd)
    );
    assert_eq!(
        Expression::parse("MIT WITH"),
        Err(ParseExpressionError::UnexpectedEnd)
    );
    assert_eq!(
        Expression::parse("MIT/"),
        Err(ParseExpressionError::UnexpectedEnd)
    );
    assert_eq!(
        Expression::parse("("),
        Err(ParseExpressionError::UnexpectedEnd)
    );
}

#[test]
fn rejects_tokens_in_the_wrong_place() {
    assert_eq!(
        Expression::parse("MIT Apache-2.0"),
        Err(ParseExpressionError::UnexpectedToken { index: 4 })
    );
    assert_eq!(
        Expression::parse("OR MIT"),
        Err(ParseExpressionError::UnexpectedToken { index: 0 })
    );
    assert_eq!(
        Expression::parse("MIT OR OR ISC"),
        Err(ParseExpressionError::UnexpectedToken { index: 7 })
    );
    assert_eq!(
        Expression::parse(")"),
        Err(ParseExpressionError::UnexpectedToken { index: 0 })
    );
    assert_eq!(
        Expression::parse("MIT)"),
        Err(ParseExpressionError::UnexpectedToken { index: 3 })
    );
    assert_eq!(
        Expression::parse("()"),
        Err(ParseExpressionError::UnexpectedToken { index: 1 })
    );
    assert_eq!(
        Expression::parse("MIT WITH AND"),
        Err(ParseExpressionError::UnexpectedToken { index: 9 })
    );
    assert_eq!(
        Expression::parse("MIT WITH (x)"),
        Err(ParseExpressionError::UnexpectedToken { index: 9 })
    );
    assert_eq!(
        Expression::parse("MIT WITH bad!"),
        Err(ParseExpressionError::UnexpectedToken { index: 9 })
    );
    assert_eq!(
        Expression::parse("mit or apache-2.0"),
        Err(ParseExpressionError::UnexpectedToken { index: 4 })
    );
}

#[test]
fn rejects_a_malformed_identifier() {
    assert_eq!(
        Expression::parse("MIT_2"),
        Err(ParseExpressionError::InvalidIdentifier { index: 0 })
    );
    assert_eq!(
        Expression::parse("MIT OR Apache 2"),
        Err(ParseExpressionError::UnexpectedToken { index: 14 })
    );
    assert_eq!(
        Expression::parse("+"),
        Err(ParseExpressionError::InvalidIdentifier { index: 0 })
    );
    assert_eq!(
        Expression::parse("é"),
        Err(ParseExpressionError::InvalidIdentifier { index: 0 })
    );
}

#[test]
fn rejects_an_unclosed_parenthesis() {
    assert_eq!(
        Expression::parse("(MIT OR ISC"),
        Err(ParseExpressionError::UnclosedParenthesis { index: 0 })
    );
    assert_eq!(
        Expression::parse("MIT AND (ISC"),
        Err(ParseExpressionError::UnclosedParenthesis { index: 8 })
    );
}
