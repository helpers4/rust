// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// A continuous-integration service, as recognized by [`detect`](super::detect).
///
/// # Examples
///
/// ```
/// use helpers4::ci::Provider;
///
/// assert_eq!(Provider::GitHubActions.name(), "GitHub Actions");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Provider {
    /// GitHub Actions.
    GitHubActions,
    /// GitLab CI/CD.
    GitLabCi,
    /// CircleCI.
    CircleCi,
    /// Travis CI.
    TravisCi,
    /// Buildkite.
    Buildkite,
    /// Azure Pipelines.
    AzurePipelines,
    /// Bitbucket Pipelines.
    BitbucketPipelines,
    /// Drone.
    Drone,
    /// TeamCity.
    TeamCity,
    /// AppVeyor.
    AppVeyor,
    /// AWS CodeBuild.
    AwsCodeBuild,
    /// Jenkins.
    Jenkins,
    /// Netlify builds.
    Netlify,
    /// Vercel builds.
    Vercel,
    /// Cloudflare Pages builds.
    CloudflarePages,
    /// An unrecognized service that sets the conventional `CI` variable.
    Other,
}

impl Provider {
    /// The service name as its vendor writes it.
    ///
    /// # Returns
    ///
    /// For instance `"GitHub Actions"`; `"CI"` for [`Provider::Other`].
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            Self::GitHubActions => "GitHub Actions",
            Self::GitLabCi => "GitLab CI",
            Self::CircleCi => "CircleCI",
            Self::TravisCi => "Travis CI",
            Self::Buildkite => "Buildkite",
            Self::AzurePipelines => "Azure Pipelines",
            Self::BitbucketPipelines => "Bitbucket Pipelines",
            Self::Drone => "Drone",
            Self::TeamCity => "TeamCity",
            Self::AppVeyor => "AppVeyor",
            Self::AwsCodeBuild => "AWS CodeBuild",
            Self::Jenkins => "Jenkins",
            Self::Netlify => "Netlify",
            Self::Vercel => "Vercel",
            Self::CloudflarePages => "Cloudflare Pages",
            Self::Other => "CI",
        }
    }
}

#[cfg(test)]
#[path = "provider.test.rs"]
mod tests;
