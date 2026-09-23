// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn applies_the_first_function_first() {
    let f = pipe(|n: i32| n + 1, |n: i32| n * 10);
    assert_eq!(f(4), 50);
}

#[test]
fn the_functions_may_change_the_type() {
    let f = pipe(|s: &str| s.len(), |n: usize| n.to_string());
    assert_eq!(f("four"), "4");
}
