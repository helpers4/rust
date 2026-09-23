// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

use crate::future::countdown::countdown;

#[test]
fn a_ready_future_gives_its_output() {
    assert_eq!(now_or_never(countdown(0, 5)), Some(5));
}

#[test]
fn a_pending_future_gives_none() {
    assert_eq!(now_or_never(countdown(1, 5)), None);
}
