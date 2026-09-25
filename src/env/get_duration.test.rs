// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn parses_a_compound_duration() {
    assert_eq!(
        get_duration("TIMEOUT=1h30m", "TIMEOUT", Duration::ZERO),
        Duration::from_secs(5400)
    );
}

#[test]
fn missing_key_returns_the_default() {
    assert_eq!(
        get_duration("", "TIMEOUT", Duration::from_secs(30)),
        Duration::from_secs(30)
    );
}

#[test]
fn a_value_that_does_not_parse_returns_the_default() {
    assert_eq!(
        get_duration("TIMEOUT=not-a-duration", "TIMEOUT", Duration::from_secs(30)),
        Duration::from_secs(30)
    );
}
