// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn exposes_the_rejected_key() {
    assert_eq!(InvalidKeyError::new("a b").key(), "a b");
}

#[test]
fn display_quotes_the_key() {
    assert_eq!(
        InvalidKeyError::new("a b").to_string(),
        "invalid environment variable name: \"a b\""
    );
}

#[test]
fn is_a_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(InvalidKeyError::new("x y"));
    assert!(err.source().is_none());
}
