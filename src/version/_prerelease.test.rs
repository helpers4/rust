// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn numbers_are_digits_without_leading_zeros() {
    assert_eq!(number("major", "0"), Ok(0));
    assert_eq!(number("major", "10"), Ok(10));
    assert_eq!(number("minor", "18446744073709551615"), Ok(u64::MAX));
    assert_eq!(
        number("minor", "01"),
        Err(ParseVersionError::LeadingZero { component: "minor" })
    );
    assert_eq!(
        number("patch", ""),
        Err(ParseVersionError::InvalidNumber { component: "patch" })
    );
    assert_eq!(
        number("patch", "1a"),
        Err(ParseVersionError::InvalidNumber { component: "patch" })
    );
    assert_eq!(
        number("patch", "-1"),
        Err(ParseVersionError::InvalidNumber { component: "patch" })
    );
    assert_eq!(
        number("major", "18446744073709551616"),
        Err(ParseVersionError::InvalidNumber { component: "major" })
    );
}

#[test]
fn identifiers_are_alphanumeric_or_hyphen() {
    assert_eq!(check_identifiers("alpha.1.x-y", true), Ok(()));
    assert_eq!(check_identifiers("0", true), Ok(()));
    assert_eq!(check_identifiers("007", false), Ok(()));
    assert_eq!(
        check_identifiers("a..b", true),
        Err(ParseVersionError::InvalidIdentifier)
    );
    assert_eq!(
        check_identifiers("", true),
        Err(ParseVersionError::InvalidIdentifier)
    );
    assert_eq!(
        check_identifiers("a_b", true),
        Err(ParseVersionError::InvalidIdentifier)
    );
    assert_eq!(
        check_identifiers("é", false),
        Err(ParseVersionError::InvalidIdentifier)
    );
}

#[test]
fn a_numeric_prerelease_identifier_has_no_leading_zero() {
    assert_eq!(
        check_identifiers("alpha.01", true),
        Err(ParseVersionError::LeadingZero {
            component: "prerelease"
        })
    );
    // Alphanumeric identifiers may start with a zero.
    assert_eq!(check_identifiers("0a", true), Ok(()));
}

fn order(a: &str, b: &str) -> Ordering {
    cmp_prerelease(Some(a), Some(b))
}

#[test]
fn a_release_is_greater_than_its_prerelease() {
    assert_eq!(cmp_prerelease(None, Some("alpha")), Ordering::Greater);
    assert_eq!(cmp_prerelease(Some("alpha"), None), Ordering::Less);
    assert_eq!(cmp_prerelease(None, None), Ordering::Equal);
}

#[test]
fn follows_the_semver_precedence_example() {
    // 1.0.0-alpha < 1.0.0-alpha.1 < 1.0.0-alpha.beta < 1.0.0-beta < 1.0.0-beta.2 <
    // 1.0.0-beta.11 < 1.0.0-rc.1
    let chain = [
        "alpha",
        "alpha.1",
        "alpha.beta",
        "beta",
        "beta.2",
        "beta.11",
        "rc.1",
    ];
    for pair in chain.windows(2) {
        assert_eq!(
            order(pair[0], pair[1]),
            Ordering::Less,
            "{} < {}",
            pair[0],
            pair[1]
        );
        assert_eq!(order(pair[1], pair[0]), Ordering::Greater);
    }
    assert_eq!(order("beta.2", "beta.2"), Ordering::Equal);
}

#[test]
fn numeric_identifiers_compare_numerically_and_below_text() {
    assert_eq!(order("2", "10"), Ordering::Less);
    assert_eq!(order("99", "100"), Ordering::Less);
    assert_eq!(order("1", "a"), Ordering::Less);
    assert_eq!(order("a", "1"), Ordering::Greater);
    assert_eq!(order("a", "b"), Ordering::Less);
    assert_eq!(order("A", "a"), Ordering::Less);
}
