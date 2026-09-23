// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Category;

/// What is known about one license: its SPDX identifier, its name and its family.
///
/// Get it with [`lookup`](super::lookup).
///
/// # Examples
///
/// ```
/// use helpers4::license::{lookup, Category};
///
/// let info = lookup("apache-2.0").unwrap();
/// assert_eq!(info.id(), "Apache-2.0");
/// assert_eq!(info.name(), "Apache License 2.0");
/// assert_eq!(info.category(), Category::Permissive);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LicenseInfo {
    id: &'static str,
    name: &'static str,
    category: Category,
}

impl LicenseInfo {
    pub(crate) fn new(id: &'static str, name: &'static str, category: Category) -> Self {
        Self { id, name, category }
    }

    /// The SPDX identifier in its canonical case.
    ///
    /// # Returns
    ///
    /// For instance `"Apache-2.0"`.
    #[must_use]
    pub fn id(&self) -> &'static str {
        self.id
    }

    /// The full name of the license.
    ///
    /// # Returns
    ///
    /// For instance `"Apache License 2.0"`.
    #[must_use]
    pub fn name(&self) -> &'static str {
        self.name
    }

    /// The family the license belongs to.
    ///
    /// # Returns
    ///
    /// The [`Category`].
    #[must_use]
    pub fn category(&self) -> Category {
        self.category
    }
}

#[cfg(test)]
#[path = "license_info.test.rs"]
mod tests;
