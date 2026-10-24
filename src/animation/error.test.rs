// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn says_what_is_wrong() {
    assert_eq!(
        ParseEasingError::UnknownFunction.to_string(),
        "not a CSS easing"
    );
    assert_eq!(
        ParseEasingError::InvalidArguments.to_string(),
        "invalid arguments for a CSS easing function"
    );
}

#[test]
fn is_a_std_error() {
    fn is_error<E: std::error::Error>(_: &E) {}
    is_error(&ParseEasingError::UnknownFunction);
}
