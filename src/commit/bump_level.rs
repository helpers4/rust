// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// How much a set of changes raises a semantic version.
///
/// Ordered from the smallest to the largest, so `max` of several levels is the one to apply.
///
/// # Examples
///
/// ```
/// use helpers4::commit::Bump;
///
/// assert!(Bump::Major > Bump::Minor && Bump::Minor > Bump::Patch && Bump::Patch > Bump::None);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bump {
    /// Nothing that affects the version (docs, tests, chores...).
    None,
    /// A backwards-compatible fix.
    Patch,
    /// A backwards-compatible feature.
    Minor,
    /// A breaking change.
    Major,
}
