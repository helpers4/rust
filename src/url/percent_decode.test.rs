// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn decodes_escapes_in_either_case() {
    assert_eq!(percent_decode("%41%4a%4A").unwrap(), "AJJ");
    assert_eq!(percent_decode("a%20b").unwrap(), "a b");
}

#[test]
fn decodes_multibyte_sequences() {
    assert_eq!(percent_decode("caf%C3%A9").unwrap(), "café");
    assert_eq!(percent_decode("%E2%82%AC").unwrap(), "€");
}

#[test]
fn a_plus_stays_a_plus() {
    assert_eq!(percent_decode("a+b").unwrap(), "a+b");
}

#[test]
fn borrows_when_there_is_no_escape() {
    assert!(matches!(
        percent_decode("plain").unwrap(),
        Cow::Borrowed("plain")
    ));
}

#[test]
fn rejects_an_incomplete_or_non_hex_escape() {
    assert_eq!(
        percent_decode("100%"),
        Err(PercentDecodeError::InvalidEscape { index: 3 })
    );
    assert_eq!(
        percent_decode("a%2"),
        Err(PercentDecodeError::InvalidEscape { index: 1 })
    );
    assert_eq!(
        percent_decode("%zz"),
        Err(PercentDecodeError::InvalidEscape { index: 0 })
    );
    assert_eq!(
        percent_decode("%4z"),
        Err(PercentDecodeError::InvalidEscape { index: 0 })
    );
    assert_eq!(
        percent_decode("%z4"),
        Err(PercentDecodeError::InvalidEscape { index: 0 })
    );
}

#[test]
fn rejects_bytes_that_are_not_utf8() {
    assert_eq!(percent_decode("%FF"), Err(PercentDecodeError::InvalidUtf8));
    assert_eq!(percent_decode("%C3"), Err(PercentDecodeError::InvalidUtf8));
}

#[test]
fn a_hex_digit_outside_the_ascii_range_is_not_a_digit() {
    assert_eq!(
        percent_decode("%é1"),
        Err(PercentDecodeError::InvalidEscape { index: 0 })
    );
}
