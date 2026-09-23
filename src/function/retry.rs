// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Runs `operation` until it succeeds, at most `attempts` times, and returns the first success or
/// the last error.
///
/// `operation` receives the number of the attempt, starting at `1`, so it can wait before a retry
/// (for instance with [`backoff`](super::backoff)) or log it: this helper never sleeps, so it
/// works the same in a test as in production. `attempts` of `0` is treated as `1`: an operation
/// is always tried once.
///
/// # Arguments
///
/// - `attempts` - The most times to run `operation`.
/// - `operation` - The work to try, given the attempt number.
///
/// # Errors
///
/// The error of the last attempt, when every attempt failed.
///
/// # Examples
///
/// ```
/// use helpers4::function::retry;
///
/// let result = retry(3, |attempt| if attempt < 3 { Err("not yet") } else { Ok(attempt) });
/// assert_eq!(result, Ok(3));
///
/// let failed: Result<u32, &str> = retry(2, |_| Err("always"));
/// assert_eq!(failed, Err("always"));
/// ```
pub fn retry<T, E>(attempts: u32, mut operation: impl FnMut(u32) -> Result<T, E>) -> Result<T, E> {
    let attempts = attempts.max(1);
    let mut attempt = 1;
    loop {
        match operation(attempt) {
            Ok(value) => return Ok(value),
            Err(error) if attempt >= attempts => return Err(error),
            Err(_) => attempt += 1,
        }
    }
}

#[cfg(test)]
#[path = "retry.test.rs"]
mod tests;

#[cfg(test)]
#[path = "retry.spec.rs"]
mod spec;
