// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn a_failure_beats_everything() {
    assert_eq!(
        overall(&[
            Status::Success,
            Status::Failure,
            Status::Pending,
            Status::Cancelled
        ]),
        Status::Failure
    );
}

#[test]
fn then_a_cancellation() {
    assert_eq!(
        overall(&[Status::Success, Status::Cancelled, Status::Pending]),
        Status::Cancelled
    );
}

#[test]
fn then_anything_pending() {
    assert_eq!(
        overall(&[Status::Success, Status::Pending, Status::Skipped]),
        Status::Pending
    );
}

#[test]
fn success_ignores_skipped_jobs() {
    assert_eq!(
        overall(&[Status::Success, Status::Skipped]),
        Status::Success
    );
    assert_eq!(overall(&[Status::Success]), Status::Success);
}

#[test]
fn nothing_ran_is_skipped() {
    assert_eq!(
        overall(&[Status::Skipped, Status::Skipped]),
        Status::Skipped
    );
    assert_eq!(overall(&[]), Status::Skipped);
}
