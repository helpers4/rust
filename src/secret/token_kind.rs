// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// A well-known kind of credential, as recognized by [`detect`](super::detect).
///
/// # Examples
///
/// ```
/// use helpers4::secret::{detect, TokenKind};
///
/// assert_eq!(detect("AKIAIOSFODNN7EXAMPLE"), Some(TokenKind::AwsAccessKey));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TokenKind {
    /// A GitHub token: `ghp_`, `gho_`, `ghu_`, `ghs_` or `ghr_` followed by 36 characters, or a
    /// fine-grained `github_pat_` token.
    GitHub,
    /// An AWS access key ID: `AKIA` or `ASIA` followed by 16 upper-case letters or digits.
    AwsAccessKey,
    /// A Slack token: `xoxa-`, `xoxb-`, `xoxp-`, `xoxr-` or `xoxs-` followed by a long body.
    Slack,
    /// A Stripe secret or restricted key: `sk_live_` or `rk_live_` followed by 24 or more
    /// letters and digits.
    Stripe,
    /// A Google API key: `AIza` followed by 35 characters.
    GoogleApiKey,
    /// An npm access token: `npm_` followed by 36 letters and digits.
    Npm,
    /// A JSON Web Token: three base64url parts separated by dots, the first starting `eyJ`.
    Jwt,
    /// The header line of a PEM private key: `-----BEGIN ... PRIVATE KEY-----`.
    PrivateKey,
}

impl TokenKind {
    /// A short human-readable name.
    ///
    /// # Returns
    ///
    /// For instance `"GitHub token"` or `"AWS access key"`.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::GitHub => "GitHub token",
            Self::AwsAccessKey => "AWS access key",
            Self::Slack => "Slack token",
            Self::Stripe => "Stripe key",
            Self::GoogleApiKey => "Google API key",
            Self::Npm => "npm token",
            Self::Jwt => "JSON Web Token",
            Self::PrivateKey => "private key",
        }
    }
}

#[cfg(test)]
#[path = "token_kind.test.rs"]
mod tests;
