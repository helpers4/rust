// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;

/// Why a string is not a CSS easing.
///
/// # Examples
///
/// ```
/// use helpers4::animation::{Easing, ParseEasingError};
///
/// assert_eq!(Easing::parse("bounce"), Err(ParseEasingError::UnknownFunction));
/// assert_eq!(Easing::parse("steps(0)"), Err(ParseEasingError::InvalidArguments));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseEasingError {
    /// Neither an easing keyword nor `cubic-bezier()`, `steps()` or `linear()`.
    UnknownFunction,
    /// A known function with arguments CSS does not accept (a wrong count, a number out of range,
    /// a malformed stop).
    InvalidArguments,
}

impl fmt::Display for ParseEasingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownFunction => write!(f, "not a CSS easing"),
            Self::InvalidArguments => write!(f, "invalid arguments for a CSS easing function"),
        }
    }
}

impl std::error::Error for ParseEasingError {}

#[cfg(test)]
#[path = "error.test.rs"]
mod tests;
