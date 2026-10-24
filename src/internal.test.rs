// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::css_number::trimmed;

#[test]
fn drops_trailing_zeros_and_the_dot() {
    assert_eq!(trimmed(12.0, 1), "12");
    assert_eq!(trimmed(0.5, 3), "0.5");
    assert_eq!(trimmed(12.34, 1), "12.3");
    assert_eq!(trimmed(1.0, 3), "1");
    assert_eq!(trimmed(0.0, 3), "0");
    assert_eq!(trimmed(100.0, 0), "100");
}

#[test]
fn rounds_to_the_given_places() {
    assert_eq!(trimmed(0.50196, 3), "0.502");
    assert_eq!(trimmed(359.96, 1), "360");
}

#[test]
fn never_writes_a_negative_zero() {
    assert_eq!(trimmed(-0.0, 1), "0");
    assert_eq!(trimmed(-0.00001, 4), "0"); // a small negative value that rounds to zero
    assert_eq!(trimmed(-0.04, 1), "0");
    assert_eq!(trimmed(-0.5, 1), "-0.5");
    assert_eq!(trimmed(-12.0, 0), "-12");
}
