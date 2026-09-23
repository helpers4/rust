// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn checks_a_version_against_a_requirement() {
    assert_eq!(satisfies("1.4.2", "^1.2"), Ok(true));
    assert_eq!(satisfies("2.0.0", "^1.2"), Ok(false));
    assert_eq!(satisfies("v1.4", "~1.4"), Ok(true));
}

#[test]
fn reports_a_bad_version_or_requirement() {
    assert_eq!(satisfies("", "^1"), Err(ParseVersionError::Empty));
    assert_eq!(satisfies("1.0.0", ""), Err(ParseVersionError::Empty));
    assert_eq!(
        satisfies("1.0.0", "^"),
        Err(ParseVersionError::InvalidComparator)
    );
}
