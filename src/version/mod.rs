// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Semantic Versioning 2.0.0: parsing, precedence, bumps and Cargo-style requirements.
//!
//! [`Version`] is a strict semver version (with a lenient parser for `v1.2`), [`VersionReq`] a requirement
//! such as `^1.2` or `>=1.0, <2.0` using Cargo's rules, and `compare` / `satisfies` are the one-line
//! versions for strings. Bad input is a typed error, never a guess.

mod _prerelease;
mod compare;
mod error;
mod satisfies;
mod semantic_version;
mod version_req;

pub use compare::compare;
pub use error::ParseVersionError;
pub use satisfies::satisfies;
pub use semantic_version::Version;
pub use version_req::VersionReq;
