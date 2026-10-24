// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn every_error_says_what_is_wrong() {
    assert_eq!(
        ParseUuidError::InvalidLength { found: 3 }.to_string(),
        "a UUID has 36 characters (or 32 without hyphens), found 3"
    );
    assert_eq!(
        ParseUuidError::InvalidCharacter { index: 8 }.to_string(),
        "invalid character at byte 8 in a UUID"
    );
    assert_eq!(
        ParseUlidError::InvalidLength { found: 5 }.to_string(),
        "a ULID has 26 characters, found 5"
    );
    assert_eq!(
        ParseUlidError::InvalidCharacter { index: 2 }.to_string(),
        "invalid character at byte 2 in a ULID"
    );
    assert_eq!(
        ParseUlidError::Overflow.to_string(),
        "a ULID starts with a digit from 0 to 7"
    );
    assert_eq!(
        AlphabetError::TooShort { found: 1 }.to_string(),
        "an alphabet needs at least 2 characters, found 1"
    );
    assert_eq!(
        AlphabetError::TooLong { found: 300 }.to_string(),
        "an alphabet has at most 256 characters, found 300"
    );
    assert_eq!(
        AlphabetError::Duplicate { character: 'x' }.to_string(),
        "the character 'x' appears twice in the alphabet"
    );
}

#[test]
fn errors_are_std_errors() {
    fn is_error<E: std::error::Error>(_: &E) {}
    is_error(&ParseUuidError::InvalidCharacter { index: 0 });
    is_error(&ParseUlidError::Overflow);
    is_error(&AlphabetError::TooShort { found: 0 });
}
