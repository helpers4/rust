// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn v(text: &str) -> Version {
    Version::parse(text).unwrap()
}

fn req(text: &str) -> VersionReq {
    VersionReq::parse(text).unwrap()
}

/// `(requirement, [(version, should it match)])`
fn check(requirement: &str, cases: &[(&str, bool)]) {
    let req = req(requirement);
    for (version, expected) in cases {
        assert_eq!(
            req.matches(&v(version)),
            *expected,
            "{requirement} vs {version}"
        );
    }
}

#[test]
fn caret_keeps_the_leftmost_nonzero_number() {
    check(
        "^1.2.3",
        &[
            ("1.2.3", true),
            ("1.9.0", true),
            ("1.2.2", false),
            ("2.0.0", false),
            ("0.9.9", false),
        ],
    );
    check(
        "^1.2",
        &[
            ("1.2.0", true),
            ("1.9.9", true),
            ("1.1.9", false),
            ("2.0.0", false),
        ],
    );
    check(
        "^1",
        &[
            ("1.0.0", true),
            ("1.9.9", true),
            ("2.0.0", false),
            ("0.9.0", false),
        ],
    );
    check(
        "^0.2.3",
        &[
            ("0.2.3", true),
            ("0.2.9", true),
            ("0.2.2", false),
            ("0.3.0", false),
        ],
    );
    check(
        "^0.2",
        &[
            ("0.2.0", true),
            ("0.2.9", true),
            ("0.3.0", false),
            ("0.1.9", false),
        ],
    );
    check(
        "^0.0.3",
        &[
            ("0.0.3", true),
            ("0.0.4", false),
            ("0.0.2", false),
            ("0.1.0", false),
        ],
    );
    check(
        "^0.0",
        &[("0.0.0", true), ("0.0.9", true), ("0.1.0", false)],
    );
    check("^0", &[("0.0.0", true), ("0.9.9", true), ("1.0.0", false)]);
}

#[test]
fn a_bare_version_means_caret() {
    check(
        "1.2.3",
        &[("1.2.3", true), ("1.9.0", true), ("2.0.0", false)],
    );
    check("0.2.3", &[("0.2.5", true), ("0.3.0", false)]);
}

#[test]
fn tilde_allows_patch_changes_or_minor_when_the_patch_is_omitted() {
    check(
        "~1.2.3",
        &[
            ("1.2.3", true),
            ("1.2.9", true),
            ("1.2.2", false),
            ("1.3.0", false),
        ],
    );
    check(
        "~1.2",
        &[
            ("1.2.0", true),
            ("1.2.9", true),
            ("1.3.0", false),
            ("1.1.9", false),
        ],
    );
    check(
        "~1",
        &[
            ("1.0.0", true),
            ("1.9.9", true),
            ("2.0.0", false),
            ("0.9.0", false),
        ],
    );
}

#[test]
fn comparison_operators_work_on_full_versions() {
    check(
        ">1.2.3",
        &[
            ("1.2.4", true),
            ("1.3.0", true),
            ("2.0.0", true),
            ("1.2.3", false),
            ("1.2.2", false),
            ("0.9.0", false),
        ],
    );
    check(
        ">=1.2.3",
        &[("1.2.3", true), ("1.2.4", true), ("1.2.2", false)],
    );
    check(
        "<1.2.3",
        &[
            ("1.2.2", true),
            ("1.1.9", true),
            ("0.9.0", true),
            ("1.2.3", false),
            ("1.3.0", false),
            ("2.0.0", false),
        ],
    );
    check(
        "<=1.2.3",
        &[("1.2.3", true), ("1.2.2", true), ("1.2.4", false)],
    );
    check(
        "=1.2.3",
        &[
            ("1.2.3", true),
            ("1.2.4", false),
            ("1.3.3", false),
            ("2.2.3", false),
        ],
    );
}

#[test]
fn comparison_operators_work_on_partial_versions() {
    check(">1", &[("2.0.0", true), ("1.9.9", false)]);
    check(
        ">1.2",
        &[("1.3.0", true), ("1.2.9", false), ("2.0.0", true)],
    );
    check(">=1", &[("1.0.0", true), ("2.0.0", true), ("0.9.9", false)]);
    check(
        ">=1.2",
        &[("1.2.0", true), ("1.1.9", false), ("1.3.0", true)],
    );
    check("<1", &[("0.9.9", true), ("1.0.0", false)]);
    check(
        "<1.2",
        &[("1.1.9", true), ("1.2.0", false), ("0.9.0", true)],
    );
    check("<=1", &[("1.9.9", true), ("2.0.0", false)]);
    check("<=1.2", &[("1.2.9", true), ("1.3.0", false)]);
    check("=1", &[("1.5.5", true), ("2.0.0", false)]);
    check("=1.2", &[("1.2.5", true), ("1.3.0", false)]);
}

#[test]
fn wildcards_match_any_number_in_their_place() {
    check("*", &[("0.0.1", true), ("9.9.9", true)]);
    check("1.*", &[("1.0.0", true), ("1.9.9", true), ("2.0.0", false)]);
    check("1.x", &[("1.4.0", true), ("0.9.0", false)]);
    check("1.*.*", &[("1.4.0", true), ("2.0.0", false)]);
    check(
        "1.2.*",
        &[("1.2.0", true), ("1.2.9", true), ("1.3.0", false)],
    );
    check("=1.X", &[("1.1.1", true), ("2.1.1", false)]);
}

#[test]
fn several_comparators_must_all_match() {
    check(
        ">=1.0, <1.5",
        &[
            ("1.0.0", true),
            ("1.4.9", true),
            ("1.5.0", false),
            ("0.9.0", false),
        ],
    );
    check(">= 1.0 , < 1.5", &[("1.2.0", true)]);
    check("*, >=2", &[("2.0.0", true), ("1.0.0", false)]);
}

#[test]
fn prereleases_only_match_a_requirement_that_names_one() {
    check("^1.2.3", &[("1.3.0-alpha", false)]);
    check("*", &[("1.0.0-alpha", false)]);
    check(">=1.0.0", &[("1.5.0-rc.1", false)]);
    check(
        ">=1.2.3-alpha.1",
        &[
            ("1.2.3-alpha.1", true),
            ("1.2.3-alpha.2", true),
            ("1.2.3", true),
            ("1.2.3-alpha", false),
            ("1.2.4-alpha", false),
        ],
    );
    check(
        "^1.2.3-alpha",
        &[("1.2.3-beta", true), ("1.2.3-aaa", false), ("1.9.0", true)],
    );
    check(
        "~1.2.3-alpha",
        &[("1.2.3-beta", true), ("1.2.3-aaa", false)],
    );
    check(
        "<1.2.3-beta",
        &[
            ("1.2.3-alpha", true),
            ("1.2.3-beta", false),
            ("1.2.2", true),
        ],
    );
    check(
        "=1.2.3-alpha",
        &[
            ("1.2.3-alpha", true),
            ("1.2.3", false),
            ("1.2.3-beta", false),
        ],
    );
    check(
        "^0.2.3-alpha",
        &[("0.2.3-beta", true), ("0.2.3-aaa", false)],
    );
    check("^0.0.3-alpha", &[("0.0.3-beta", true)]);
}

#[test]
fn accepts_a_v_prefix_and_spaces_after_the_operator() {
    check("^v1.2", &[("1.5.0", true)]);
    check(">= v1.2.3", &[("1.2.3", true)]);
}

#[test]
fn build_metadata_of_the_checked_version_is_ignored() {
    check("=1.2.3", &[("1.2.3+build", true)]);
}

#[test]
fn rejects_an_empty_requirement() {
    assert_eq!(VersionReq::parse(""), Err(ParseVersionError::Empty));
    assert_eq!(VersionReq::parse("  "), Err(ParseVersionError::Empty));
}

#[test]
fn rejects_malformed_comparators() {
    for text in [
        "^",
        ">=",
        "1.2.3+build",
        ">1.*",
        "^1.*",
        "~1.x",
        "<=*.1",
        "1.*.3",
        "1.2.x.3",
        "1.*-alpha",
        "1-alpha",
        "1.2-alpha",
        "a",
        "1.2.3,",
        ",",
    ] {
        assert!(
            VersionReq::parse(text).is_err(),
            "{text:?} should be rejected"
        );
    }
    assert_eq!(
        VersionReq::parse("1.*.3"),
        Err(ParseVersionError::InvalidComparator)
    );
    assert_eq!(
        VersionReq::parse("1.2-alpha"),
        Err(ParseVersionError::InvalidComparator)
    );
    assert_eq!(
        VersionReq::parse(">1.*"),
        Err(ParseVersionError::InvalidComparator)
    );
    assert_eq!(
        VersionReq::parse("1.2.3+b"),
        Err(ParseVersionError::InvalidComparator)
    );
}

#[test]
fn reports_number_and_identifier_errors() {
    assert_eq!(
        VersionReq::parse("a"),
        Err(ParseVersionError::InvalidNumber { component: "major" })
    );
    assert_eq!(
        VersionReq::parse("1.b"),
        Err(ParseVersionError::InvalidNumber { component: "minor" })
    );
    assert_eq!(
        VersionReq::parse("1.2.c"),
        Err(ParseVersionError::InvalidNumber { component: "patch" })
    );
    assert_eq!(
        VersionReq::parse("01.2.3"),
        Err(ParseVersionError::LeadingZero { component: "major" })
    );
    assert_eq!(
        VersionReq::parse("1.2.3-a..b"),
        Err(ParseVersionError::InvalidIdentifier)
    );
}
