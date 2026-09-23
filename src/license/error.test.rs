// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn display_describes_each_variant() {
    assert_eq!(
        ParseExpressionError::Empty.to_string(),
        "empty license expression"
    );
    assert_eq!(
        ParseExpressionError::UnexpectedEnd.to_string(),
        "the expression ends too early"
    );
    assert_eq!(
        ParseExpressionError::UnexpectedToken { index: 4 }.to_string(),
        "unexpected token at byte 4"
    );
    assert_eq!(
        ParseExpressionError::InvalidIdentifier { index: 2 }.to_string(),
        "invalid license identifier at byte 2"
    );
    assert_eq!(
        ParseExpressionError::UnclosedParenthesis { index: 0 }.to_string(),
        "unclosed parenthesis at byte 0"
    );
}

#[test]
fn is_a_std_error() {
    let err: Box<dyn std::error::Error> = Box::new(ParseExpressionError::Empty);
    assert!(err.source().is_none());
}
