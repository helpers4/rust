// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

use std::cell::Cell;
use std::rc::Rc;

/// Every test builds its memoizer here, so the generic code is instantiated (and covered) once.
fn counting(calls: &Rc<Cell<usize>>) -> Memoize<u32, u32, impl FnMut(&u32) -> u32> {
    let calls = Rc::clone(calls);
    Memoize::new(move |n: &u32| {
        calls.set(calls.get() + 1);
        n * 2
    })
}

#[test]
fn computes_once_per_distinct_argument() {
    let calls = Rc::new(Cell::new(0));
    let mut double = counting(&calls);
    assert_eq!(double.call(4), 8);
    assert_eq!(double.call(4), 8);
    assert_eq!(double.call(5), 10);
    assert_eq!(double.call(4), 8);
    assert_eq!(calls.get(), 2);
}

#[test]
fn reports_what_it_remembers() {
    let calls = Rc::new(Cell::new(0));
    let mut double = counting(&calls);
    assert!(double.is_empty());
    assert_eq!(double.len(), 0);
    double.call(1);
    double.call(2);
    double.call(1);
    assert!(!double.is_empty());
    assert_eq!(double.len(), 2);
}

#[test]
fn clearing_makes_it_compute_again() {
    let calls = Rc::new(Cell::new(0));
    let mut double = counting(&calls);
    double.call(3);
    double.clear();
    assert!(double.is_empty());
    double.call(3);
    assert_eq!(calls.get(), 2);
}
