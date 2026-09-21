// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::time::Duration;

/// Formats `duration` as a short human-readable string such as `"1h 30m 5s"`.
///
/// Uses the units `d`, `h`, `m`, `s` and `ms`, largest first, and skips the ones that are zero. A
/// zero duration is `"0s"`. Anything below one millisecond is dropped (truncated, not rounded).
/// [`parse`](super::parse) reads the result back.
///
/// # Examples
///
/// ```
/// use helpers4::duration::format;
/// use std::time::Duration;
///
/// assert_eq!(format(Duration::from_secs(5405)), "1h 30m 5s");
/// assert_eq!(format(Duration::from_millis(1500)), "1s 500ms");
/// assert_eq!(format(Duration::ZERO), "0s");
/// ```
#[must_use]
pub fn format(duration: Duration) -> String {
    let secs = duration.as_secs();
    let parts = [
        (secs / 86_400, "d"),
        (secs % 86_400 / 3600, "h"),
        (secs % 3600 / 60, "m"),
        (secs % 60, "s"),
        (u64::from(duration.subsec_millis()), "ms"),
    ];
    let text = parts
        .iter()
        .filter(|(amount, _)| *amount > 0)
        .map(|(amount, unit)| std::format!("{amount}{unit}"))
        .collect::<Vec<_>>()
        .join(" ");
    if text.is_empty() {
        "0s".to_string()
    } else {
        text
    }
}

#[cfg(test)]
#[path = "format.test.rs"]
mod tests;

#[cfg(test)]
#[path = "format.spec.rs"]
mod spec;
