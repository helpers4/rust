// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn every_provider_has_its_vendor_name() {
    let names: Vec<&str> = [
        Provider::GitHubActions,
        Provider::GitLabCi,
        Provider::CircleCi,
        Provider::TravisCi,
        Provider::Buildkite,
        Provider::AzurePipelines,
        Provider::BitbucketPipelines,
        Provider::Drone,
        Provider::TeamCity,
        Provider::AppVeyor,
        Provider::AwsCodeBuild,
        Provider::Jenkins,
        Provider::Netlify,
        Provider::Vercel,
        Provider::CloudflarePages,
        Provider::Other,
    ]
    .iter()
    .map(|p| p.name())
    .collect();
    assert_eq!(
        names,
        [
            "GitHub Actions",
            "GitLab CI",
            "CircleCI",
            "Travis CI",
            "Buildkite",
            "Azure Pipelines",
            "Bitbucket Pipelines",
            "Drone",
            "TeamCity",
            "AppVeyor",
            "AWS CodeBuild",
            "Jenkins",
            "Netlify",
            "Vercel",
            "Cloudflare Pages",
            "CI"
        ]
    );
}
