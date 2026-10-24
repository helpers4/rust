// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use std::time::Duration;

#[test]
fn counts_milliseconds_since_the_epoch() {
    assert_eq!(unix_millis(UNIX_EPOCH), 0);
    assert_eq!(
        unix_millis(UNIX_EPOCH + Duration::from_millis(1_700_000_000_123)),
        1_700_000_000_123
    );
}

#[test]
fn a_time_before_1970_is_zero() {
    assert_eq!(unix_millis(UNIX_EPOCH - Duration::from_secs(1)), 0);
}

#[test]
fn the_current_time_is_after_2023() {
    assert!(unix_millis(SystemTime::now()) > 1_700_000_000_000);
}
