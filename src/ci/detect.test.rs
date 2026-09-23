// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

use std::collections::HashMap;

/// A lookup over a fixed environment, built here so every test shares one closure type.
fn env(pairs: &[(&str, &str)]) -> HashMap<String, String> {
    pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn detected(pairs: &[(&str, &str)]) -> Option<Provider> {
    let vars = env(pairs);
    detect(&|name| vars.get(name).cloned())
}

#[test]
fn recognizes_every_service_by_its_variable() {
    for (variable, provider) in MARKERS {
        assert_eq!(
            detected(&[(variable, "true")]),
            Some(provider),
            "{variable}"
        );
    }
}

#[test]
fn a_bare_ci_variable_is_an_unknown_service() {
    assert_eq!(detected(&[("CI", "true")]), Some(Provider::Other));
    assert_eq!(detected(&[("CI", "1")]), Some(Provider::Other));
}

#[test]
fn a_known_service_wins_over_the_generic_variable() {
    assert_eq!(
        detected(&[("CI", "true"), ("GITLAB_CI", "true")]),
        Some(Provider::GitLabCi)
    );
}

#[test]
fn the_most_specific_service_wins_when_several_are_set() {
    assert_eq!(
        detected(&[("JENKINS_URL", "http://j"), ("GITHUB_ACTIONS", "true")]),
        Some(Provider::GitHubActions)
    );
}

#[test]
fn no_variable_means_no_ci() {
    assert_eq!(detected(&[]), None);
    assert_eq!(detected(&[("HOME", "/home/me")]), None);
}

#[test]
fn falsy_values_do_not_count() {
    for value in ["", "0", "false", "FALSE", "False"] {
        assert_eq!(detected(&[("CI", value)]), None, "{value:?}");
        assert_eq!(detected(&[("GITHUB_ACTIONS", value)]), None, "{value:?}");
    }
}

#[test]
fn truthiness_of_a_missing_value() {
    assert!(!is_truthy(None));
    assert!(is_truthy(Some("yes")));
}
