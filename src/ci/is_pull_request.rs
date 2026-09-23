// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::detect::is_truthy;
use super::{Provider, detect};

/// Whether the current CI run was triggered by a pull request (or merge request).
///
/// It looks at the variable each service sets for that: `GITHUB_EVENT_NAME` (`pull_request` or
/// `pull_request_target`), `CI_MERGE_REQUEST_IID` on GitLab, `CIRCLE_PULL_REQUEST`,
/// `TRAVIS_PULL_REQUEST`, `BUILDKITE_PULL_REQUEST`, `SYSTEM_PULLREQUEST_PULLREQUESTID` on Azure,
/// `BITBUCKET_PR_ID`, `DRONE_PULL_REQUEST`, `APPVEYOR_PULL_REQUEST_NUMBER`, `PULL_REQUEST` on
/// Netlify and `VERCEL_GIT_PULL_REQUEST_ID`. Services without a known variable (Jenkins,
/// TeamCity, ...) and any environment outside CI give `false`.
///
/// # Arguments
///
/// - `get` - Looks a variable up by name.
///
/// # Returns
///
/// `true` for a pull request run.
///
/// # Examples
///
/// ```
/// use helpers4::ci::is_pull_request;
///
/// let env = |name: &str| match name {
///     "GITHUB_ACTIONS" => Some("true".to_string()),
///     "GITHUB_EVENT_NAME" => Some("pull_request".to_string()),
///     _ => None,
/// };
/// assert!(is_pull_request(&env));
/// ```
#[must_use]
pub fn is_pull_request(get: &dyn Fn(&str) -> Option<String>) -> bool {
    let truthy = |name: &str| is_truthy(get(name).as_deref());
    match detect(get) {
        Some(Provider::GitHubActions) => matches!(
            get("GITHUB_EVENT_NAME").as_deref(),
            Some("pull_request" | "pull_request_target")
        ),
        Some(Provider::GitLabCi) => truthy("CI_MERGE_REQUEST_IID"),
        Some(Provider::CircleCi) => truthy("CIRCLE_PULL_REQUEST"),
        Some(Provider::TravisCi) => truthy("TRAVIS_PULL_REQUEST"),
        Some(Provider::Buildkite) => truthy("BUILDKITE_PULL_REQUEST"),
        Some(Provider::AzurePipelines) => truthy("SYSTEM_PULLREQUEST_PULLREQUESTID"),
        Some(Provider::BitbucketPipelines) => truthy("BITBUCKET_PR_ID"),
        Some(Provider::Drone) => truthy("DRONE_PULL_REQUEST"),
        Some(Provider::AppVeyor) => truthy("APPVEYOR_PULL_REQUEST_NUMBER"),
        Some(Provider::Netlify) => truthy("PULL_REQUEST"),
        Some(Provider::Vercel) => truthy("VERCEL_GIT_PULL_REQUEST_ID"),
        _ => false,
    }
}

#[cfg(test)]
#[path = "is_pull_request.test.rs"]
mod tests;
