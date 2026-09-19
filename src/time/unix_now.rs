// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::time::SystemTime;

use super::_epoch::since_epoch;
use super::error::ClockError;

/// The current time as whole seconds since the Unix epoch.
///
/// # Errors
///
/// [`ClockError`] when the system clock is set before 1970. Do not turn that into `0`: a token
/// expiry checked against `0` would look valid forever.
///
/// # Examples
///
/// ```
/// use helpers4::time::unix_now;
///
/// let now = unix_now()?;
/// assert!(now > 1_700_000_000); // after November 2023
/// # Ok::<(), helpers4::time::ClockError>(())
/// ```
pub fn unix_now() -> Result<u64, ClockError> {
    since_epoch(SystemTime::now()).map(|elapsed| elapsed.as_secs())
}

#[cfg(test)]
#[path = "unix_now.test.rs"]
mod tests;
