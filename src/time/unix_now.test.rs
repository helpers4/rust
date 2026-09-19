// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn is_a_plausible_current_time() {
    assert!(unix_now().unwrap() > 1_700_000_000);
}

#[test]
fn does_not_go_backwards_between_two_calls() {
    let first = unix_now().unwrap();
    let second = unix_now().unwrap();
    assert!(second >= first);
}
