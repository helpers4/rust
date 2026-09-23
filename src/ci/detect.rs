// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Provider;

/// The variables that identify each service, most specific first. A variable counts when it is
/// set to something other than an empty string, `0` or `false`.
const MARKERS: [(&str, Provider); 15] = [
    ("GITHUB_ACTIONS", Provider::GitHubActions),
    ("GITLAB_CI", Provider::GitLabCi),
    ("CIRCLECI", Provider::CircleCi),
    ("TRAVIS", Provider::TravisCi),
    ("BUILDKITE", Provider::Buildkite),
    ("TF_BUILD", Provider::AzurePipelines),
    ("BITBUCKET_BUILD_NUMBER", Provider::BitbucketPipelines),
    ("DRONE", Provider::Drone),
    ("TEAMCITY_VERSION", Provider::TeamCity),
    ("APPVEYOR", Provider::AppVeyor),
    ("CODEBUILD_BUILD_ARN", Provider::AwsCodeBuild),
    ("JENKINS_URL", Provider::Jenkins),
    ("NETLIFY", Provider::Netlify),
    ("VERCEL", Provider::Vercel),
    ("CF_PAGES", Provider::CloudflarePages),
];

/// Which CI service the environment belongs to, if any.
///
/// The environment is a parameter, not read behind your back: pass a lookup such as
/// `&|name| std::env::var(name).ok()`, or a closure over a map in a test. The known services are
/// recognized by their own variables (`GITHUB_ACTIONS`, `GITLAB_CI`, `CIRCLECI`, ...); any other
/// environment with a truthy `CI` variable is [`Provider::Other`]. A variable is truthy unless it
/// is empty, `0` or `false` (case-insensitive).
///
/// # Arguments
///
/// - `get` - Looks a variable up by name.
///
/// # Returns
///
/// The [`Provider`], or `None` outside CI.
///
/// # Examples
///
/// ```
/// use helpers4::ci::{detect, Provider};
///
/// let env = |name: &str| (name == "GITHUB_ACTIONS").then(|| "true".to_string());
/// assert_eq!(detect(&env), Some(Provider::GitHubActions));
/// assert_eq!(detect(&|_: &str| None), None);
/// ```
#[must_use]
pub fn detect(get: &dyn Fn(&str) -> Option<String>) -> Option<Provider> {
    MARKERS
        .iter()
        .find(|(name, _)| is_truthy(get(name).as_deref()))
        .map(|(_, provider)| *provider)
        .or_else(|| is_truthy(get("CI").as_deref()).then_some(Provider::Other))
}

/// Whether a variable is set to something other than nothing, `0` or `false`.
pub(crate) fn is_truthy(value: Option<&str>) -> bool {
    value.is_some_and(|v| !v.is_empty() && v != "0" && !v.eq_ignore_ascii_case("false"))
}

#[cfg(test)]
#[path = "detect.test.rs"]
mod tests;
