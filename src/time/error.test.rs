// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn exposes_how_far_behind_the_clock_is() {
    assert_eq!(
        ClockError::new(Duration::from_secs(5)).behind(),
        Duration::from_secs(5)
    );
}

#[test]
fn display_names_the_offset() {
    assert_eq!(
        ClockError::new(Duration::from_secs(5)).to_string(),
        "system clock is 5s before the Unix epoch"
    );
}

#[test]
fn is_a_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(ClockError::new(Duration::ZERO));
    assert!(err.source().is_none());
}
