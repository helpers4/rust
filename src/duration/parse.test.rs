// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn secs(n: u64) -> Duration {
    Duration::from_secs(n)
}

#[test]
fn parses_a_single_unit() {
    assert_eq!(parse("500ms"), Ok(Duration::from_millis(500)));
    assert_eq!(parse("45s"), Ok(secs(45)));
    assert_eq!(parse("2m"), Ok(secs(120)));
    assert_eq!(parse("3h"), Ok(secs(10_800)));
    assert_eq!(parse("2d"), Ok(secs(172_800)));
    assert_eq!(parse("1w"), Ok(secs(604_800)));
}

#[test]
fn sums_several_pairs_in_any_order() {
    assert_eq!(parse("1h30m"), Ok(secs(5400)));
    assert_eq!(parse("30m1h"), Ok(secs(5400)));
    assert_eq!(parse("1m1m"), Ok(secs(120)));
    assert_eq!(parse("1s500ms"), Ok(Duration::from_millis(1500)));
}

#[test]
fn allows_whitespace_between_pairs_and_around() {
    assert_eq!(parse(" 1h \t 30m\n"), Ok(secs(5400)));
}

#[test]
fn zero_is_a_valid_duration() {
    assert_eq!(parse("0s"), Ok(Duration::ZERO));
}

#[test]
fn empty_input_is_an_error() {
    assert_eq!(parse(""), Err(ParseDurationError::Empty));
    assert_eq!(parse("  \n"), Err(ParseDurationError::Empty));
}

#[test]
fn something_other_than_a_number_is_an_error() {
    assert_eq!(
        parse("h"),
        Err(ParseDurationError::ExpectedNumber { index: 0 })
    );
    assert_eq!(
        parse("1h -5m"),
        Err(ParseDurationError::ExpectedNumber { index: 3 })
    );
    assert_eq!(
        parse("é"),
        Err(ParseDurationError::ExpectedNumber { index: 0 })
    );
}

#[test]
fn a_number_without_a_unit_is_an_error() {
    assert_eq!(
        parse("1.5h"),
        Err(ParseDurationError::MissingUnit { index: 1 })
    );
    assert_eq!(
        parse("90"),
        Err(ParseDurationError::MissingUnit { index: 2 })
    );
    assert_eq!(
        parse("1h 30"),
        Err(ParseDurationError::MissingUnit { index: 5 })
    );
    assert_eq!(
        parse("5 m"),
        Err(ParseDurationError::MissingUnit { index: 1 })
    );
}

#[test]
fn an_unknown_unit_is_an_error() {
    assert_eq!(
        parse("5x"),
        Err(ParseDurationError::UnknownUnit { index: 1 })
    );
    assert_eq!(
        parse("1h5min"),
        Err(ParseDurationError::UnknownUnit { index: 3 })
    );
    assert_eq!(
        parse("1H"),
        Err(ParseDurationError::UnknownUnit { index: 1 })
    );
}

#[test]
fn a_total_that_does_not_fit_is_an_error() {
    assert_eq!(
        parse("99999999999999999999s"),
        Err(ParseDurationError::Overflow)
    );
    assert_eq!(
        parse("18446744073709551615w"),
        Err(ParseDurationError::Overflow)
    );
    assert_eq!(
        parse("18446744073709551615d"),
        Err(ParseDurationError::Overflow)
    );
    assert_eq!(
        parse("18446744073709551615h"),
        Err(ParseDurationError::Overflow)
    );
    assert_eq!(
        parse("18446744073709551615m"),
        Err(ParseDurationError::Overflow)
    );
    assert_eq!(
        parse("18446744073709551615s1s"),
        Err(ParseDurationError::Overflow)
    );
}
