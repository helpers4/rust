// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn encodes_reserved_characters() {
    assert_eq!(percent_encode("a b&c=d/e?f#g"), "a%20b%26c%3Dd%2Fe%3Ff%23g");
}

#[test]
fn encodes_every_byte_of_a_multibyte_character() {
    assert_eq!(percent_encode("é"), "%C3%A9");
    assert_eq!(percent_encode("€"), "%E2%82%AC");
}

#[test]
fn keeps_unreserved_characters() {
    assert_eq!(percent_encode("AZaz09-._~"), "AZaz09-._~");
}

#[test]
fn encodes_the_percent_sign_itself() {
    assert_eq!(percent_encode("100%"), "100%25");
}

#[test]
fn borrows_when_there_is_nothing_to_encode() {
    assert!(matches!(percent_encode("plain"), Cow::Borrowed("plain")));
    assert!(matches!(percent_encode(""), Cow::Borrowed("")));
}
