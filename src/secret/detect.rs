// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::TokenKind;

/// Recognizes a well-known credential format: the whole of `token` must look like one.
///
/// The check is by prefix, alphabet and length (GitHub, AWS, Slack, Stripe, Google, npm, JWT, and
/// the header of a PEM private key): no network call, so it says a string *looks like* a token, not
/// that it is valid, and it will miss formats it does not know. Use [`scan`](super::scan) to
/// search a whole text.
///
/// # Arguments
///
/// - `token` - The string to check, a single word without surrounding quotes.
///
/// # Returns
///
/// The [`TokenKind`], or `None` when the string matches no known format.
///
/// # Examples
///
/// ```
/// use helpers4::secret::{detect, TokenKind};
///
/// assert_eq!(detect("ghp_0123456789abcdefghijklmnopqrstuvwxyz"), Some(TokenKind::GitHub));
/// assert_eq!(detect("just-a-word"), None);
/// ```
#[must_use]
pub fn detect(token: &str) -> Option<TokenKind> {
    if is_private_key_header(token) {
        return Some(TokenKind::PrivateKey);
    }
    let alnum = |text: &str| text.bytes().all(|b| b.is_ascii_alphanumeric());
    let after = |prefix: &str| token.strip_prefix(prefix);
    if let Some(body) = ["ghp_", "gho_", "ghu_", "ghs_", "ghr_"]
        .iter()
        .find_map(|p| after(p))
    {
        if body.len() == 36 && alnum(body) {
            return Some(TokenKind::GitHub);
        }
    }
    if let Some(body) = after("github_pat_") {
        if body.len() >= 22 && body.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_') {
            return Some(TokenKind::GitHub);
        }
    }
    if let Some(body) = ["AKIA", "ASIA"].iter().find_map(|p| after(p)) {
        if body.len() == 16
            && body
                .bytes()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
        {
            return Some(TokenKind::AwsAccessKey);
        }
    }
    if let Some(body) = ["xoxa-", "xoxb-", "xoxp-", "xoxr-", "xoxs-"]
        .iter()
        .find_map(|p| after(p))
    {
        if body.len() >= 10 && body.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-') {
            return Some(TokenKind::Slack);
        }
    }
    if let Some(body) = ["sk_live_", "rk_live_"].iter().find_map(|p| after(p)) {
        if body.len() >= 24 && alnum(body) {
            return Some(TokenKind::Stripe);
        }
    }
    if let Some(body) = after("AIza") {
        if body.len() == 35
            && body
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            return Some(TokenKind::GoogleApiKey);
        }
    }
    if let Some(body) = after("npm_") {
        if body.len() == 36 && alnum(body) {
            return Some(TokenKind::Npm);
        }
    }
    if is_jwt(token) {
        return Some(TokenKind::Jwt);
    }
    None
}

/// `-----BEGIN PRIVATE KEY-----`, `-----BEGIN RSA PRIVATE KEY-----`, ...
pub(crate) fn is_private_key_header(text: &str) -> bool {
    text.strip_prefix("-----BEGIN ")
        .and_then(|rest| rest.strip_suffix("-----"))
        .is_some_and(|label| label.ends_with("PRIVATE KEY"))
}

fn is_jwt(token: &str) -> bool {
    let parts: Vec<&str> = token.split('.').collect();
    parts.len() == 3
        && parts[0].starts_with("eyJ")
        && parts.iter().all(|part| {
            !part.is_empty()
                && part
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        })
}

#[cfg(test)]
#[path = "detect.test.rs"]
mod tests;

#[cfg(test)]
#[path = "detect.spec.rs"]
mod spec;
