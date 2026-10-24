// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use std::error::Error;

#[test]
fn wraps_the_system_error_and_says_what_failed() {
    let error = EntropyError::from(getrandom::Error::UNSUPPORTED);
    assert!(
        error
            .to_string()
            .starts_with("the operating system could not provide random bytes: ")
    );
    assert!(error.source().is_some());
}

#[test]
fn is_comparable_and_copyable() {
    let a = EntropyError::from(getrandom::Error::UNSUPPORTED);
    let b = a;
    assert_eq!(a, b);
    assert_ne!(a, EntropyError::from(getrandom::Error::UNEXPECTED));
    assert!(format!("{a:?}").contains("EntropyError"));
}
