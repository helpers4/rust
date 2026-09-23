// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

use std::collections::HashMap;

fn is_pr(pairs: &[(&str, &str)]) -> bool {
    let vars: HashMap<String, String> = pairs
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();
    is_pull_request(&|name| vars.get(name).cloned())
}

#[test]
fn github_actions_looks_at_the_event_name() {
    assert!(is_pr(&[
        ("GITHUB_ACTIONS", "true"),
        ("GITHUB_EVENT_NAME", "pull_request")
    ]));
    assert!(is_pr(&[
        ("GITHUB_ACTIONS", "true"),
        ("GITHUB_EVENT_NAME", "pull_request_target")
    ]));
    assert!(!is_pr(&[
        ("GITHUB_ACTIONS", "true"),
        ("GITHUB_EVENT_NAME", "push")
    ]));
    assert!(!is_pr(&[("GITHUB_ACTIONS", "true")]));
}

#[test]
fn other_services_use_their_own_variable() {
    for (service, variable) in [
        ("GITLAB_CI", "CI_MERGE_REQUEST_IID"),
        ("CIRCLECI", "CIRCLE_PULL_REQUEST"),
        ("TRAVIS", "TRAVIS_PULL_REQUEST"),
        ("BUILDKITE", "BUILDKITE_PULL_REQUEST"),
        ("TF_BUILD", "SYSTEM_PULLREQUEST_PULLREQUESTID"),
        ("BITBUCKET_BUILD_NUMBER", "BITBUCKET_PR_ID"),
        ("DRONE", "DRONE_PULL_REQUEST"),
        ("APPVEYOR", "APPVEYOR_PULL_REQUEST_NUMBER"),
        ("NETLIFY", "PULL_REQUEST"),
        ("VERCEL", "VERCEL_GIT_PULL_REQUEST_ID"),
    ] {
        assert!(is_pr(&[(service, "true"), (variable, "42")]), "{service}");
        assert!(!is_pr(&[(service, "true")]), "{service} without a PR");
        assert!(
            !is_pr(&[(service, "true"), (variable, "false")]),
            "{service} with false"
        );
    }
}

#[test]
fn services_without_a_known_variable_are_never_pull_requests() {
    assert!(!is_pr(&[("JENKINS_URL", "http://j")]));
    assert!(!is_pr(&[("TEAMCITY_VERSION", "1")]));
    assert!(!is_pr(&[("CI", "true")]));
}

#[test]
fn outside_ci_it_is_false() {
    assert!(!is_pr(&[]));
    assert!(!is_pr(&[("GITHUB_EVENT_NAME", "pull_request")]));
}
