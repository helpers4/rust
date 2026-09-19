// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! The one place that turns a `SystemTime` into a duration since the epoch. Not re-exported.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::error::ClockError;

pub(crate) fn since_epoch(time: SystemTime) -> Result<Duration, ClockError> {
    time.duration_since(UNIX_EPOCH)
        .map_err(|before| ClockError::new(before.duration()))
}

#[cfg(test)]
#[path = "_epoch.test.rs"]
mod tests;
