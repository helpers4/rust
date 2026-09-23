// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn v(text: &str) -> Version {
    Version::parse(text).unwrap()
}

#[test]
fn parses_the_three_numbers() {
    let version = v("1.4.2");
    assert_eq!(
        (version.major(), version.minor(), version.patch()),
        (1, 4, 2)
    );
    assert_eq!((version.pre(), version.build()), (None, None));
    assert!(!version.is_prerelease());
}

#[test]
fn parses_prerelease_and_build() {
    let version = v("1.0.0-alpha.1+sha.5114f85");
    assert_eq!(version.pre(), Some("alpha.1"));
    assert_eq!(version.build(), Some("sha.5114f85"));
    assert!(version.is_prerelease());
    assert_eq!(v("1.0.0+20130313144700").build(), Some("20130313144700"));
    assert_eq!(v("1.0.0-x-y.z").pre(), Some("x-y.z"));
}

#[test]
fn a_hyphen_inside_the_build_is_not_a_prerelease() {
    let version = v("1.0.0+a-b");
    assert_eq!((version.pre(), version.build()), (None, Some("a-b")));
}

#[test]
fn strict_parsing_rejects_partial_and_prefixed_versions() {
    assert_eq!(Version::parse(""), Err(ParseVersionError::Empty));
    assert_eq!(
        Version::parse("1"),
        Err(ParseVersionError::MissingComponent { component: "minor" })
    );
    assert_eq!(
        Version::parse("1.2"),
        Err(ParseVersionError::MissingComponent { component: "patch" })
    );
    assert_eq!(
        Version::parse("v1.2.3"),
        Err(ParseVersionError::InvalidNumber { component: "major" })
    );
    assert_eq!(
        Version::parse(" 1.2.3"),
        Err(ParseVersionError::InvalidNumber { component: "major" })
    );
}

#[test]
fn rejects_bad_numbers() {
    assert_eq!(
        Version::parse("a.2.3"),
        Err(ParseVersionError::InvalidNumber { component: "major" })
    );
    assert_eq!(
        Version::parse("1.b.3"),
        Err(ParseVersionError::InvalidNumber { component: "minor" })
    );
    assert_eq!(
        Version::parse("1.2.c"),
        Err(ParseVersionError::InvalidNumber { component: "patch" })
    );
    assert_eq!(
        Version::parse("1.2.3.4"),
        Err(ParseVersionError::InvalidNumber { component: "patch" })
    );
    assert_eq!(
        Version::parse("1.2."),
        Err(ParseVersionError::InvalidNumber { component: "patch" })
    );
    assert_eq!(
        Version::parse("-1.2.3"),
        Err(ParseVersionError::InvalidNumber { component: "major" })
    );
}

#[test]
fn rejects_leading_zeros() {
    assert_eq!(
        Version::parse("01.2.3"),
        Err(ParseVersionError::LeadingZero { component: "major" })
    );
    assert_eq!(
        Version::parse("1.02.3"),
        Err(ParseVersionError::LeadingZero { component: "minor" })
    );
    assert_eq!(
        Version::parse("1.2.03"),
        Err(ParseVersionError::LeadingZero { component: "patch" })
    );
    assert_eq!(
        Version::parse("1.2.3-alpha.01"),
        Err(ParseVersionError::LeadingZero {
            component: "prerelease"
        })
    );
    assert!(Version::parse("1.2.3+001").is_ok());
}

#[test]
fn rejects_bad_identifiers() {
    assert_eq!(
        Version::parse("1.2.3-"),
        Err(ParseVersionError::InvalidIdentifier)
    );
    assert_eq!(
        Version::parse("1.2.3+"),
        Err(ParseVersionError::InvalidIdentifier)
    );
    assert_eq!(
        Version::parse("1.2.3-a..b"),
        Err(ParseVersionError::InvalidIdentifier)
    );
    assert_eq!(
        Version::parse("1.2.3-a_b"),
        Err(ParseVersionError::InvalidIdentifier)
    );
    assert_eq!(
        Version::parse("1.2.3+a b"),
        Err(ParseVersionError::InvalidIdentifier)
    );
}

#[test]
fn lenient_parsing_fills_in_the_blanks() {
    assert_eq!(Version::parse_lenient("v1.2"), Ok(Version::new(1, 2, 0)));
    assert_eq!(Version::parse_lenient("V3"), Ok(Version::new(3, 0, 0)));
    assert_eq!(
        Version::parse_lenient("  1.2.3  "),
        Ok(Version::new(1, 2, 3))
    );
    assert_eq!(Version::parse_lenient("2-rc.1"), Ok(v("2.0.0-rc.1")));
    assert_eq!(Version::parse_lenient("1.2.3+b"), Ok(v("1.2.3+b")));
}

#[test]
fn lenient_parsing_still_rejects_nonsense() {
    assert_eq!(Version::parse_lenient(""), Err(ParseVersionError::Empty));
    assert_eq!(Version::parse_lenient("  "), Err(ParseVersionError::Empty));
    assert_eq!(Version::parse_lenient("v"), Err(ParseVersionError::Empty));
    assert_eq!(
        Version::parse_lenient("x.1"),
        Err(ParseVersionError::InvalidNumber { component: "major" })
    );
    assert_eq!(
        Version::parse_lenient("1.x"),
        Err(ParseVersionError::InvalidNumber { component: "minor" })
    );
    assert_eq!(
        Version::parse_lenient("1.2.x"),
        Err(ParseVersionError::InvalidNumber { component: "patch" })
    );
}

#[test]
fn displays_the_normalized_form() {
    assert_eq!(Version::new(1, 2, 3).to_string(), "1.2.3");
    assert_eq!(v("1.0.0-alpha.1+b.5").to_string(), "1.0.0-alpha.1+b.5");
    assert_eq!(v("1.0.0+b").to_string(), "1.0.0+b");
    assert_eq!("1.2.3".parse::<Version>(), Ok(Version::new(1, 2, 3)));
}

#[test]
fn orders_by_semver_precedence() {
    let chain = [
        "1.0.0-alpha",
        "1.0.0-alpha.1",
        "1.0.0-alpha.beta",
        "1.0.0-beta",
        "1.0.0-beta.2",
        "1.0.0-beta.11",
        "1.0.0-rc.1",
        "1.0.0",
        "1.0.1",
        "1.1.0",
        "2.0.0",
        "10.0.0",
    ];
    for pair in chain.windows(2) {
        assert!(v(pair[0]) < v(pair[1]), "{} < {}", pair[0], pair[1]);
        assert_eq!(v(pair[1]).precedence(&v(pair[0])), Ordering::Greater);
    }
}

#[test]
fn build_metadata_is_ignored_by_precedence_but_breaks_ties_in_ord() {
    assert_eq!(v("1.0.0+a").precedence(&v("1.0.0+b")), Ordering::Equal);
    assert_ne!(v("1.0.0+a"), v("1.0.0+b"));
    assert!(v("1.0.0+a") < v("1.0.0+b"));
    assert!(v("1.0.0") < v("1.0.0+a"));
    assert_eq!(v("1.0.0+a").cmp(&v("1.0.0+a")), Ordering::Equal);
}

#[test]
fn bumps_reset_the_lower_parts_and_drop_prerelease_and_build() {
    let version = v("1.4.2-rc.1+b");
    assert_eq!(version.bump_major(), Version::new(2, 0, 0));
    assert_eq!(version.bump_minor(), Version::new(1, 5, 0));
    assert_eq!(version.bump_patch(), Version::new(1, 4, 3));
}

#[test]
fn bumps_saturate() {
    let max = Version::new(u64::MAX, u64::MAX, u64::MAX);
    assert_eq!(max.bump_major(), Version::new(u64::MAX, 0, 0));
    assert_eq!(max.bump_minor(), Version::new(u64::MAX, u64::MAX, 0));
    assert_eq!(max.bump_patch(), max);
}
