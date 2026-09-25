// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn recognizes_every_truthy_word_case_insensitively() {
    for word in ["true", "TRUE", "1", "yes", "Yes", "on"] {
        assert!(
            get_bool(&format!("KEY={word}"), "KEY", false),
            "{word} should be true"
        );
    }
}

#[test]
fn recognizes_every_falsy_word_case_insensitively() {
    for word in ["false", "FALSE", "0", "no", "No", "off"] {
        assert!(
            !get_bool(&format!("KEY={word}"), "KEY", true),
            "{word} should be false"
        );
    }
}

#[test]
fn missing_key_returns_the_default() {
    assert!(get_bool("", "KEY", true));
    assert!(!get_bool("", "KEY", false));
}

#[test]
fn an_unrecognized_value_returns_the_default() {
    assert!(get_bool("KEY=maybe", "KEY", true));
    assert!(!get_bool("KEY=maybe", "KEY", false));
}
