// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Status;

/// The single status that sums up several: the worst one wins.
///
/// A failure beats everything, then a cancellation, then anything still pending; when everything
/// that ran succeeded the result is success, skipped jobs not counting against it. A list where
/// every job was skipped, and an empty list, sum up to [`Status::Skipped`] (nothing ran).
///
/// # Arguments
///
/// - `statuses` - The statuses of the jobs.
///
/// # Returns
///
/// The overall status.
///
/// # Examples
///
/// ```
/// use helpers4::ci::{overall, Status};
///
/// assert_eq!(overall(&[Status::Success, Status::Skipped]), Status::Success);
/// assert_eq!(overall(&[Status::Success, Status::Failure, Status::Pending]), Status::Failure);
/// assert_eq!(overall(&[]), Status::Skipped);
/// ```
#[must_use]
pub fn overall(statuses: &[Status]) -> Status {
    let has = |wanted: Status| statuses.contains(&wanted);
    if has(Status::Failure) {
        Status::Failure
    } else if has(Status::Cancelled) {
        Status::Cancelled
    } else if has(Status::Pending) {
        Status::Pending
    } else if has(Status::Success) {
        Status::Success
    } else {
        Status::Skipped
    }
}

#[cfg(test)]
#[path = "overall.test.rs"]
mod tests;

#[cfg(test)]
#[path = "overall.spec.rs"]
mod spec;
