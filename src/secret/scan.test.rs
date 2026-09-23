// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

const AWS: &str = "AKIAIOSFODNN7EXAMPLE";
const GITHUB: &str = "ghp_0123456789abcdefghijklmnopqrstuvwxyz";

fn found(text: &str) -> Vec<(TokenKind, &str)> {
    scan(text)
        .iter()
        .map(|finding| (finding.kind(), &text[finding.start()..finding.end()]))
        .collect()
}

#[test]
fn finds_a_token_and_reports_its_range() {
    let text = format!("key={AWS}");
    let findings = scan(&text);
    assert_eq!(findings.len(), 1);
    assert_eq!((findings[0].start(), findings[0].end()), (4, 4 + AWS.len()));
    assert_eq!(findings[0].kind(), TokenKind::AwsAccessKey);
}

#[test]
fn ignores_quotes_and_punctuation_around_a_token() {
    assert_eq!(
        found(&format!("\"{AWS}\",")),
        vec![(TokenKind::AwsAccessKey, AWS)]
    );
    assert_eq!(
        found(&format!("({GITHUB})")),
        vec![(TokenKind::GitHub, GITHUB)]
    );
}

#[test]
fn finds_several_tokens_in_order_across_lines() {
    let text = format!("a {AWS}\nnothing\nb: {GITHUB}");
    assert_eq!(
        found(&text),
        vec![(TokenKind::AwsAccessKey, AWS), (TokenKind::GitHub, GITHUB)]
    );
}

#[test]
fn a_token_at_the_very_end_of_the_text_is_found() {
    assert_eq!(found(AWS), vec![(TokenKind::AwsAccessKey, AWS)]);
}

#[test]
fn finds_a_private_key_header_line() {
    let text = "before\n  -----BEGIN RSA PRIVATE KEY-----  \nMIIB...\n";
    assert_eq!(
        found(text),
        vec![(TokenKind::PrivateKey, "-----BEGIN RSA PRIVATE KEY-----")]
    );
}

#[test]
fn a_jwt_with_dots_is_one_word() {
    let jwt = "eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjMifQ.c2lnbmF0dXJl";
    assert_eq!(
        found(&format!("Authorization: Bearer {jwt}")),
        vec![(TokenKind::Jwt, jwt)]
    );
}

#[test]
fn ordinary_text_has_no_findings() {
    assert!(scan("").is_empty());
    assert!(scan("just some text, with words and numbers 123").is_empty());
}

#[test]
fn offsets_stay_correct_after_multibyte_text() {
    let text = format!("é {AWS}");
    assert_eq!(found(&text), vec![(TokenKind::AwsAccessKey, AWS)]);
}
