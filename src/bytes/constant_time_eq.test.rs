// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn equal_slices_match() {
    assert!(constant_time_eq(b"secret", b"secret"));
    assert!(constant_time_eq(&[], &[]));
}

#[test]
fn a_different_byte_is_detected_wherever_it_is() {
    assert!(!constant_time_eq(b"secret", b"Secret"));
    assert!(!constant_time_eq(b"secret", b"secreT"));
}

#[test]
fn different_lengths_never_match() {
    assert!(!constant_time_eq(b"secret", b"secre"));
    assert!(!constant_time_eq(b"", b"a"));
}

#[test]
fn differences_that_would_cancel_out_are_still_detected() {
    // `1 ^ 0` twice would cancel under `^=`; `1 & 2` is zero under `&`.
    assert!(!constant_time_eq(&[1, 1], &[0, 0]));
    assert!(!constant_time_eq(&[1], &[2]));
}
