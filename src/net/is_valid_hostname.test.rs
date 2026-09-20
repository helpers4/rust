// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn accepts_ordinary_hostnames() {
    for s in [
        "localhost",
        "example.com",
        "a.b.c.d",
        "my-host.example.org",
        "123.example",
        "xn--bcher-kva.example",
        "A-Z.example",
        "example.com.",
    ] {
        assert_eq!(is_valid_hostname(s), Ok(()), "{s}");
    }
}

#[test]
fn empty_input_and_empty_labels_are_rejected() {
    assert_eq!(is_valid_hostname(""), Err(HostnameError::Empty));
    assert_eq!(is_valid_hostname("."), Err(HostnameError::EmptyLabel));
    assert_eq!(
        is_valid_hostname(".example.com"),
        Err(HostnameError::EmptyLabel)
    );
    assert_eq!(is_valid_hostname("a..b"), Err(HostnameError::EmptyLabel));
    assert_eq!(
        is_valid_hostname("example.com.."),
        Err(HostnameError::EmptyLabel)
    );
}

#[test]
fn length_limits_are_exact() {
    let label63 = "a".repeat(63);
    assert_eq!(is_valid_hostname(&label63), Ok(()));
    assert_eq!(
        is_valid_hostname(&"a".repeat(64)),
        Err(HostnameError::LabelTooLong)
    );

    let ok253 = format!("{label63}.{label63}.{label63}.{}", "a".repeat(61));
    assert_eq!(ok253.len(), 253);
    assert_eq!(is_valid_hostname(&ok253), Ok(()));
    assert_eq!(is_valid_hostname(&format!("{ok253}.")), Ok(()));
    let too_long = format!("{label63}.{label63}.{label63}.{}", "a".repeat(62));
    assert_eq!(too_long.len(), 254);
    assert_eq!(is_valid_hostname(&too_long), Err(HostnameError::TooLong));
}

#[test]
fn invalid_characters_report_their_byte_offset() {
    assert_eq!(
        is_valid_hostname("a_b.example"),
        Err(HostnameError::InvalidChar {
            index: 1,
            found: '_'
        })
    );
    assert_eq!(
        is_valid_hostname("ok.exa mple"),
        Err(HostnameError::InvalidChar {
            index: 6,
            found: ' '
        })
    );
    assert_eq!(
        is_valid_hostname("bücher.example"),
        Err(HostnameError::InvalidChar {
            index: 1,
            found: 'ü'
        })
    );
}

#[test]
fn hyphens_are_only_allowed_inside_a_label() {
    assert_eq!(
        is_valid_hostname("-a.example"),
        Err(HostnameError::HyphenEdge)
    );
    assert_eq!(
        is_valid_hostname("a-.example"),
        Err(HostnameError::HyphenEdge)
    );
    assert_eq!(
        is_valid_hostname("example.-"),
        Err(HostnameError::HyphenEdge)
    );
    assert_eq!(is_valid_hostname("a--b.example"), Ok(()));
}

#[test]
fn a_numeric_last_label_is_rejected() {
    for s in [
        "2130706433",
        "127.0.0.1",
        "127.1",
        "0x7f.1",
        "0177.0.0.1",
        "0x7f000001",
        "0xdeadbeef",
        "0x",
        "0X1F",
        "example.123",
        "example.123.",
        "1",
    ] {
        assert_eq!(
            is_valid_hostname(s),
            Err(HostnameError::NumericLastLabel),
            "{s}"
        );
    }
}

#[test]
fn numeric_labels_elsewhere_and_near_misses_are_accepted() {
    for s in [
        "123.example",
        "0x7f.example",
        "example.0xg",
        "example.1a",
        "a1",
        "0xg",
        "example.0x1g",
    ] {
        assert_eq!(is_valid_hostname(s), Ok(()), "{s}");
    }
}
