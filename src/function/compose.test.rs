// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn applies_the_inner_function_first() {
    let f = compose(|n: i32| n * 10, |n: i32| n + 1);
    assert_eq!(f(4), 50); // (4 + 1) * 10, not 4 * 10 + 1
}

#[test]
fn the_functions_may_change_the_type() {
    let f = compose(|n: usize| n.to_string(), |s: &str| s.len());
    assert_eq!(f("four"), "4");
}
