// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn replaces_every_occurrence() {
    assert_eq!(redact("a abc b abc", &["abc"]), "a [REDACTED] b [REDACTED]");
}

#[test]
fn hides_several_secrets() {
    assert_eq!(
        redact("user=bob pass=xyz", &["bob", "xyz"]),
        "user=[REDACTED] pass=[REDACTED]"
    );
}

#[test]
fn a_longer_secret_containing_a_shorter_one_is_hidden_whole() {
    assert_eq!(
        redact("token-abc-123", &["abc", "token-abc-123"]),
        "[REDACTED]"
    );
    assert_eq!(
        redact("token-abc-123", &["token-abc-123", "abc"]),
        "[REDACTED]"
    );
}

#[test]
fn ignores_empty_secrets_and_missing_ones() {
    assert_eq!(redact("nothing here", &["", "absent"]), "nothing here");
    assert_eq!(redact("text", &[]), "text");
}

#[test]
fn works_on_multibyte_text() {
    assert_eq!(
        redact("mot de passe: sécrèt!", &["sécrèt"]),
        "mot de passe: [REDACTED]!"
    );
}
