// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn every_kind_has_a_name() {
    let names: Vec<&str> = [
        TokenKind::GitHub,
        TokenKind::AwsAccessKey,
        TokenKind::Slack,
        TokenKind::Stripe,
        TokenKind::GoogleApiKey,
        TokenKind::Npm,
        TokenKind::Jwt,
        TokenKind::PrivateKey,
    ]
    .iter()
    .map(|kind| kind.name())
    .collect();
    assert_eq!(
        names,
        [
            "GitHub token",
            "AWS access key",
            "Slack token",
            "Stripe key",
            "Google API key",
            "npm token",
            "JSON Web Token",
            "private key"
        ]
    );
}
