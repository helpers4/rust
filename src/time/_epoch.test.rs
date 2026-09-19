// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn the_epoch_itself_is_zero() {
    assert_eq!(since_epoch(UNIX_EPOCH), Ok(Duration::ZERO));
}

#[test]
fn a_later_time_is_the_offset() {
    assert_eq!(
        since_epoch(UNIX_EPOCH + Duration::from_millis(1500)),
        Ok(Duration::from_millis(1500))
    );
}

#[test]
fn a_time_before_the_epoch_is_an_error_with_the_gap() {
    let before = UNIX_EPOCH - Duration::from_secs(5);
    assert_eq!(
        since_epoch(before),
        Err(ClockError::new(Duration::from_secs(5)))
    );
}
