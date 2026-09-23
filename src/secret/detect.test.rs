// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

const GITHUB: &str = "ghp_0123456789abcdefghijklmnopqrstuvwxyz";

#[test]
fn recognizes_github_tokens() {
    assert_eq!(detect(GITHUB), Some(TokenKind::GitHub));
    for prefix in ["gho_", "ghu_", "ghs_", "ghr_"] {
        assert_eq!(
            detect(&format!("{prefix}{}", &GITHUB[4..])),
            Some(TokenKind::GitHub)
        );
    }
    assert_eq!(
        detect("github_pat_11ABCDEFG0abcdefghijkl_0123456789"),
        Some(TokenKind::GitHub)
    );
}

#[test]
fn rejects_github_lookalikes() {
    assert_eq!(detect("ghp_short"), None);
    assert_eq!(detect(&format!("{GITHUB}0")), None);
    assert_eq!(detect("ghp_0123456789abcdefghijklmnopqrstuvwxy!"), None);
    assert_eq!(detect("github_pat_short"), None);
    assert_eq!(detect("github_pat_0123456789abcdefghijkl!"), None);
}

#[test]
fn recognizes_aws_access_keys() {
    assert_eq!(
        detect("AKIAIOSFODNN7EXAMPLE"),
        Some(TokenKind::AwsAccessKey)
    );
    assert_eq!(
        detect("ASIAIOSFODNN7EXAMPLE"),
        Some(TokenKind::AwsAccessKey)
    );
    assert_eq!(detect("AKIAIOSFODNN7EXAMPL"), None);
    assert_eq!(detect("AKIAiosfodnn7example"), None);
}

#[test]
fn recognizes_slack_tokens() {
    assert_eq!(detect("xoxb-1234567890-abcdefghij"), Some(TokenKind::Slack));
    assert_eq!(detect("xoxp-1234567890"), Some(TokenKind::Slack));
    assert_eq!(detect("xoxb-short"), None);
    assert_eq!(detect("xoxb-1234567890-abcdef!ghij"), None);
}

/// Builds a fixture at run time. A literal that matches the format of a real key would trip secret
/// scanners (and GitHub's push protection), even though these are made-up values.
fn stripe(prefix: &str, body: &str) -> String {
    format!("{prefix}{body}")
}

#[test]
fn recognizes_stripe_keys() {
    let body = "0123456789abcdefghijklmn";
    assert_eq!(detect(&stripe("sk_live_", body)), Some(TokenKind::Stripe));
    assert_eq!(detect(&stripe("rk_live_", body)), Some(TokenKind::Stripe));
    assert_eq!(detect(&stripe("sk_test_", body)), None);
    assert_eq!(detect(&stripe("sk_live_", "short")), None);
}

#[test]
fn recognizes_google_api_keys() {
    assert_eq!(
        detect("AIzaSyA-1234567890abcdefghijklmnopqrstu"),
        Some(TokenKind::GoogleApiKey)
    );
    assert_eq!(detect("AIzaShort"), None);
    assert_eq!(detect("AIzaSyA-1234567890abcdefghijklmnopqrst!"), None);
}

#[test]
fn recognizes_npm_tokens() {
    assert_eq!(
        detect("npm_0123456789abcdefghijklmnopqrstuvwxyz"),
        Some(TokenKind::Npm)
    );
    assert_eq!(detect("npm_short"), None);
}

#[test]
fn recognizes_jwts() {
    assert_eq!(
        detect("eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjMifQ.c2lnbmF0dXJl"),
        Some(TokenKind::Jwt)
    );
    assert_eq!(detect("eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjMifQ"), None);
    assert_eq!(detect("abc.def.ghi"), None);
    assert_eq!(detect("eyJa..c2ln"), None);
    assert_eq!(detect("eyJa.b!.c"), None);
}

#[test]
fn recognizes_private_key_headers() {
    assert_eq!(
        detect("-----BEGIN PRIVATE KEY-----"),
        Some(TokenKind::PrivateKey)
    );
    assert_eq!(
        detect("-----BEGIN RSA PRIVATE KEY-----"),
        Some(TokenKind::PrivateKey)
    );
    assert_eq!(detect("-----BEGIN PUBLIC KEY-----"), None);
    assert_eq!(detect("-----BEGIN PRIVATE KEY"), None);
}

#[test]
fn ordinary_text_is_not_a_token() {
    assert_eq!(detect(""), None);
    assert_eq!(detect("hello"), None);
    assert_eq!(detect("just-a-word"), None);
}
