// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// A value that must not leak through logs, error messages or `{:?}`: printing it shows
/// `[REDACTED]` instead of the content.
///
/// The only way to read it is the explicit [`expose`](Self::expose) (or
/// [`into_inner`](Self::into_inner)), which makes every use of the secret easy to find in a code
/// review. It is not `PartialEq`, `Hash` or serializable, so it cannot be compared, used as a key
/// or written out by accident. **It does not wipe the memory when dropped**: that needs
/// unsafe code, which this crate forbids, so use a dedicated crate such as `zeroize` when a
/// value must not linger in memory.
///
/// # Examples
///
/// ```
/// use helpers4::secret::Secret;
///
/// let token = Secret::new("hunter2".to_string());
/// assert_eq!(format!("{token}"), "[REDACTED]");
/// assert_eq!(format!("{token:?}"), "Secret([REDACTED])");
/// assert_eq!(token.expose(), "hunter2");
/// ```
#[derive(Clone)]
pub struct Secret<T> {
    value: T,
}

impl<T> Secret<T> {
    /// Wraps `value`.
    ///
    /// # Arguments
    ///
    /// - `value` - The sensitive value.
    ///
    /// # Returns
    ///
    /// The wrapper, which hides the value from `Debug` and `Display`.
    #[must_use]
    pub fn new(value: T) -> Self {
        Self { value }
    }

    /// Borrows the value.
    ///
    /// # Returns
    ///
    /// A reference to the wrapped value: the one place where it is read.
    #[must_use]
    pub fn expose(&self) -> &T {
        &self.value
    }

    /// Unwraps the value.
    ///
    /// # Returns
    ///
    /// The wrapped value, no longer protected.
    #[must_use]
    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> fmt::Debug for Secret<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret([REDACTED])")
    }
}

impl<T> fmt::Display for Secret<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[REDACTED]")
    }
}

impl<T> From<T> for Secret<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

#[cfg(test)]
#[path = "secret_value.test.rs"]
mod tests;
