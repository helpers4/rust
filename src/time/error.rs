// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::fmt;
use std::time::Duration;

/// The system clock is set before the Unix epoch (1970-01-01T00:00:00Z).
///
/// Returned instead of a silent `0`: an expiry comparison against `0` would treat every token as
/// still valid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClockError {
    behind: Duration,
}

impl ClockError {
    pub(crate) fn new(behind: Duration) -> Self {
        Self { behind }
    }

    /// How far before the epoch the clock is.
    #[must_use]
    pub fn behind(&self) -> Duration {
        self.behind
    }
}

impl fmt::Display for ClockError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "system clock is {:?} before the Unix epoch", self.behind)
    }
}

impl std::error::Error for ClockError {}

#[cfg(test)]
#[path = "error.test.rs"]
mod tests;
