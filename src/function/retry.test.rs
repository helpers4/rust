// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

use std::cell::Cell;

/// Every test runs its operation through here, so the generic code is instantiated (and covered)
/// once: it succeeds on attempt `succeed_on`, or never when that is `0`.
fn run(attempts: u32, succeed_on: u32, calls: &Cell<u32>) -> Result<u32, String> {
    retry(attempts, |attempt| {
        calls.set(calls.get() + 1);
        if attempt == succeed_on {
            Ok(attempt)
        } else {
            Err(format!("failed {attempt}"))
        }
    })
}

#[test]
fn returns_the_first_success_without_more_attempts() {
    let calls = Cell::new(0);
    assert_eq!(run(5, 1, &calls), Ok(1));
    assert_eq!(calls.get(), 1);
}

#[test]
fn retries_until_it_succeeds() {
    let calls = Cell::new(0);
    assert_eq!(run(5, 3, &calls), Ok(3));
    assert_eq!(calls.get(), 3);
}

#[test]
fn gives_the_last_error_when_every_attempt_fails() {
    let calls = Cell::new(0);
    assert_eq!(run(3, 0, &calls), Err("failed 3".to_string()));
    assert_eq!(calls.get(), 3);
}

#[test]
fn succeeding_on_the_last_attempt_counts() {
    let calls = Cell::new(0);
    assert_eq!(run(3, 3, &calls), Ok(3));
}

#[test]
fn zero_attempts_still_tries_once() {
    let calls = Cell::new(0);
    assert_eq!(run(0, 0, &calls), Err("failed 1".to_string()));
    assert_eq!(calls.get(), 1);
    let calls = Cell::new(0);
    assert_eq!(run(0, 1, &calls), Ok(1));
}
