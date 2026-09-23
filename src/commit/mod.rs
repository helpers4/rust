// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Conventional Commits: parse a message, decide the version bump, validate.
//!
//! [`Commit`] parses `type(scope)!: description` with its body and footers, following the
//! specification; [`Bump`] and `bump_for` turn a list of commits into the semantic-version bump they
//! call for, `is_valid` is the yes/no shortcut, and `emoji_for` gives the gitmoji of the helpers4
//! convention.

mod bump_for;
mod bump_level;
mod conventional_commit;
mod emoji_for;
mod error;
mod is_valid;

pub use bump_for::bump_for;
pub use bump_level::Bump;
pub use conventional_commit::Commit;
pub use emoji_for::emoji_for;
pub use error::ParseCommitError;
pub use is_valid::is_valid;
