// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn is_true_inside_ci() {
    assert!(is_ci(&|name| (name == "CI").then(|| "true".to_string())));
    assert!(is_ci(
        &|name| (name == "CIRCLECI").then(|| "true".to_string())
    ));
}

#[test]
fn is_false_outside_ci() {
    assert!(!is_ci(&|_| None));
    assert!(!is_ci(&|name| (name == "CI").then(|| "false".to_string())));
}
