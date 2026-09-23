// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

use crate::future::block_on;
use crate::future::countdown::countdown;

#[test]
fn completes_with_both_outputs() {
    assert_eq!(block_on(join(countdown(0, 1), countdown(0, 2))), (1, 2));
}

#[test]
fn waits_for_the_slower_future() {
    assert_eq!(block_on(join(countdown(0, 1), countdown(4, 2))), (1, 2));
    assert_eq!(block_on(join(countdown(4, 1), countdown(0, 2))), (1, 2));
}

#[test]
fn keeps_the_output_of_the_first_finisher_while_waiting() {
    // The first future is done on the first poll; its output must survive the pending rounds.
    assert_eq!(block_on(join(countdown(0, 5), countdown(3, 6))), (5, 6));
}

#[test]
fn both_futures_make_progress_together() {
    // Sequential execution would need 3 + 3 pending rounds; together they need 3.
    let mut future = Box::pin(join(countdown(3, 1), countdown(3, 2)));
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    for _ in 0..3 {
        assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
    }
    assert_eq!(future.as_mut().poll(&mut context), Poll::Ready((1, 2)));
}
