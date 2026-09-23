// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// How a license treats the code that uses it: the main families, from most to least permissive.
///
/// # Examples
///
/// ```
/// use helpers4::license::{lookup, Category};
///
/// assert_eq!(lookup("MIT").map(|info| info.category()), Some(Category::Permissive));
/// assert_eq!(lookup("GPL-3.0-only").map(|info| info.category()), Some(Category::StrongCopyleft));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Category {
    /// No restriction beyond keeping the notice (MIT, Apache-2.0, BSD).
    Permissive,
    /// Copyright waived or dedicated to the public domain (CC0-1.0, Unlicense).
    PublicDomain,
    /// Changes to the licensed files must stay open, but a program that merely uses them need not
    /// (MPL-2.0, LGPL).
    WeakCopyleft,
    /// A program that includes the code must be released under the same license (GPL).
    StrongCopyleft,
    /// Like strong copyleft, and it also applies when the program is only offered over a network
    /// (AGPL, SSPL).
    NetworkCopyleft,
}
