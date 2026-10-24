// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn writes_known_bytes_as_lowercase_hex() {
    assert_eq!(to_hex(&[]), "");
    assert_eq!(to_hex(&[0x00, 0x0f, 0xab, 0xff]), "000fabff");
}

#[test]
fn has_two_characters_per_byte_in_lowercase_hex() {
    for len in [0, 1, 16, 32] {
        let hex = random_hex(len).unwrap_or_default();
        assert_eq!(hex.len(), len * 2);
        assert!(hex.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')));
    }
}

#[test]
fn two_draws_differ() {
    assert_ne!(random_hex(32), random_hex(32));
}
