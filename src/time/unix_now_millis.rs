// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::time::SystemTime;

use super::_epoch::since_epoch;
use super::error::ClockError;

/// The current time as milliseconds since the Unix epoch.
///
/// The value saturates at `u64::MAX`, which is some 584 million years away.
///
/// # Errors
///
/// [`ClockError`] when the system clock is set before 1970 (see [`unix_now`](super::unix_now)).
///
/// # Examples
///
/// ```
/// use helpers4::time::unix_now_millis;
///
/// assert!(unix_now_millis()? > 1_700_000_000_000);
/// # Ok::<(), helpers4::time::ClockError>(())
/// ```
pub fn unix_now_millis() -> Result<u64, ClockError> {
    since_epoch(SystemTime::now())
        .map(|elapsed| u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX))
}

#[cfg(test)]
#[path = "unix_now_millis.test.rs"]
mod tests;
