// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::time::{SystemTime, UNIX_EPOCH};

/// Milliseconds between the Unix epoch and `now`: `0` for a time before 1970, and `u64::MAX`
/// (never reached in practice) when the count does not fit.
pub(crate) fn unix_millis(now: SystemTime) -> u64 {
    now.duration_since(UNIX_EPOCH).map_or(0, |elapsed| {
        u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX)
    })
}

#[cfg(test)]
#[path = "_unix_millis.test.rs"]
mod tests;
