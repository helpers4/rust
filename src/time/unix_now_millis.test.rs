// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn is_a_plausible_current_time_in_milliseconds() {
    assert!(unix_now_millis().unwrap() > 1_700_000_000_000);
}

#[test]
fn agrees_with_the_seconds_helper() {
    let seconds = crate::time::unix_now().unwrap();
    let millis = unix_now_millis().unwrap();
    assert!(millis / 1000 >= seconds);
    assert!(millis / 1000 <= seconds + 2);
}
