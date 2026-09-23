// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn compares_numbers_numerically_not_as_text() {
    assert_eq!(compare("1.10.0", "1.9.0"), Ok(Ordering::Greater));
    assert_eq!(compare("1.9.0", "1.10.0"), Ok(Ordering::Less));
}

#[test]
fn reads_versions_leniently() {
    assert_eq!(compare("v2", "2.0.0"), Ok(Ordering::Equal));
    assert_eq!(compare("1.2", "1.2.1"), Ok(Ordering::Less));
}

#[test]
fn a_prerelease_is_below_its_release() {
    assert_eq!(compare("1.0.0-rc.1", "1.0.0"), Ok(Ordering::Less));
}

#[test]
fn ignores_build_metadata() {
    assert_eq!(compare("1.0.0+a", "1.0.0+b"), Ok(Ordering::Equal));
}

#[test]
fn reports_the_string_that_does_not_parse() {
    assert_eq!(compare("", "1.0.0"), Err(ParseVersionError::Empty));
    assert_eq!(
        compare("1.0.0", "x"),
        Err(ParseVersionError::InvalidNumber { component: "major" })
    );
}
