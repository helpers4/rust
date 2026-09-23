// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{Bump, Commit};

/// The largest version bump called for by any of `commits`.
///
/// One breaking change means [`Bump::Major`]; otherwise a `feat` means [`Bump::Minor`], a `fix`
/// [`Bump::Patch`], and commits of other types (docs, tests, chores) do not bump the version.
/// An empty list gives [`Bump::None`].
///
/// # Arguments
///
/// - `commits` - The parsed commits of a release, in any order.
///
/// # Returns
///
/// The bump to apply to the current version.
///
/// # Examples
///
/// ```
/// use helpers4::commit::{bump_for, Bump, Commit};
///
/// let commits = [
///     Commit::parse("docs: update the readme")?,
///     Commit::parse("fix: handle empty input")?,
///     Commit::parse("feat: add a helper")?,
/// ];
/// assert_eq!(bump_for(&commits), Bump::Minor);
/// # Ok::<(), helpers4::commit::ParseCommitError>(())
/// ```
#[must_use]
pub fn bump_for(commits: &[Commit]) -> Bump {
    commits.iter().map(Commit::bump).max().unwrap_or(Bump::None)
}

#[cfg(test)]
#[path = "bump_for.test.rs"]
mod tests;
