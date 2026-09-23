// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn a_bare_number_is_bytes() {
    assert_eq!(parse_size("512"), Ok(512));
    assert_eq!(parse_size("512 B"), Ok(512));
    assert_eq!(parse_size("0"), Ok(0));
}

#[test]
fn binary_units_are_powers_of_1024() {
    assert_eq!(parse_size("1KiB"), Ok(1 << 10));
    assert_eq!(parse_size("1 MiB"), Ok(1 << 20));
    assert_eq!(parse_size("1GiB"), Ok(1 << 30));
    assert_eq!(parse_size("1TiB"), Ok(1 << 40));
    assert_eq!(parse_size("1PiB"), Ok(1 << 50));
    assert_eq!(parse_size("1EiB"), Ok(1 << 60));
}

#[test]
fn single_letters_are_binary() {
    assert_eq!(parse_size("2k"), Ok(2048));
    assert_eq!(parse_size("1M"), Ok(1 << 20));
    assert_eq!(parse_size("1g"), Ok(1 << 30));
    assert_eq!(parse_size("1T"), Ok(1 << 40));
    assert_eq!(parse_size("1P"), Ok(1 << 50));
    assert_eq!(parse_size("1E"), Ok(1 << 60));
}

#[test]
fn si_units_are_powers_of_1000() {
    assert_eq!(parse_size("1kB"), Ok(1_000));
    assert_eq!(parse_size("10MB"), Ok(10_000_000));
    assert_eq!(parse_size("1GB"), Ok(1_000_000_000));
    assert_eq!(parse_size("1TB"), Ok(1_000_000_000_000));
    assert_eq!(parse_size("1PB"), Ok(1_000_000_000_000_000));
    assert_eq!(parse_size("1EB"), Ok(1_000_000_000_000_000_000));
}

#[test]
fn units_ignore_case_and_whitespace() {
    assert_eq!(parse_size("  1.5   kib  "), Ok(1536));
    assert_eq!(parse_size("1 KIB"), Ok(1024));
}

#[test]
fn fractions_are_rounded_down() {
    assert_eq!(parse_size("1.5 KiB"), Ok(1536));
    assert_eq!(parse_size("0.9 B"), Ok(0));
    assert_eq!(parse_size("1.000000000000000000009 KiB"), Ok(1024));
}

#[test]
fn empty_input_is_an_error() {
    assert_eq!(parse_size(""), Err(ParseSizeError::Empty));
    assert_eq!(parse_size("  "), Err(ParseSizeError::Empty));
}

#[test]
fn something_other_than_a_number_is_an_error() {
    assert_eq!(
        parse_size("KiB"),
        Err(ParseSizeError::ExpectedNumber { index: 0 })
    );
    assert_eq!(
        parse_size(" -1"),
        Err(ParseSizeError::ExpectedNumber { index: 1 })
    );
    assert_eq!(
        parse_size("1."),
        Err(ParseSizeError::ExpectedNumber { index: 2 })
    );
    assert_eq!(
        parse_size(".5"),
        Err(ParseSizeError::ExpectedNumber { index: 0 })
    );
}

#[test]
fn an_unknown_unit_is_an_error() {
    assert_eq!(
        parse_size("5 bytes"),
        Err(ParseSizeError::UnknownUnit { index: 2 })
    );
    assert_eq!(
        parse_size("5x"),
        Err(ParseSizeError::UnknownUnit { index: 1 })
    );
    assert_eq!(
        parse_size("5 KiB extra"),
        Err(ParseSizeError::UnknownUnit { index: 2 })
    );
}

#[test]
fn a_size_that_does_not_fit_is_an_error() {
    assert_eq!(
        parse_size("18446744073709551616"),
        Err(ParseSizeError::Overflow)
    );
    assert_eq!(parse_size("16 EiB"), Err(ParseSizeError::Overflow));
    assert_eq!(
        parse_size("999999999999999999999999999999999999999 EiB"),
        Err(ParseSizeError::Overflow)
    );
    assert_eq!(
        parse_size("340282366920938463463374607431768211456"),
        Err(ParseSizeError::Overflow)
    );
    // The multiplication overflows u128 even though the number itself parses.
    assert_eq!(
        parse_size("1000000000000000000000000000000 EiB"),
        Err(ParseSizeError::Overflow)
    );
    // The multiplication fits, the fractional part then pushes the sum over u128::MAX.
    assert_eq!(
        parse_size("340282366920938463463374607431768211.999 kB"),
        Err(ParseSizeError::Overflow)
    );
    assert_eq!(parse_size("18446744073709551615"), Ok(u64::MAX));
}
