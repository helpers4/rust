// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::detect;

/// Whether the environment looks like a CI run.
///
/// The same as `detect(get).is_some()`. The environment is a parameter: pass
/// `&|name| std::env::var(name).ok()` for the real one.
///
/// # Arguments
///
/// - `get` - Looks a variable up by name.
///
/// # Returns
///
/// `true` when a known CI service, or a truthy `CI` variable, is present.
///
/// # Examples
///
/// ```
/// use helpers4::ci::is_ci;
///
/// assert!(is_ci(&|name: &str| (name == "CI").then(|| "true".to_string())));
/// assert!(!is_ci(&|_: &str| None));
/// ```
#[must_use]
pub fn is_ci(get: &dyn Fn(&str) -> Option<String>) -> bool {
    detect(get).is_some()
}

#[cfg(test)]
#[path = "is_ci.test.rs"]
mod tests;
