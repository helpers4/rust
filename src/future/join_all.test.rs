// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

use crate::future::block_on;
use crate::future::countdown::countdown;

#[test]
fn collects_the_outputs_in_input_order() {
    let futures = vec![countdown(3, 10), countdown(0, 20), countdown(1, 30)];
    assert_eq!(block_on(join_all(futures)), vec![10, 20, 30]);
}

#[test]
fn an_empty_input_completes_at_once() {
    assert_eq!(
        block_on(join_all(Vec::<crate::future::countdown::Countdown>::new())),
        Vec::<u32>::new()
    );
}

#[test]
fn a_single_future_works() {
    assert_eq!(block_on(join_all(vec![countdown(2, 7)])), vec![7]);
}

#[test]
fn all_futures_make_progress_together() {
    let mut future = Box::pin(join_all(vec![
        countdown(3, 1),
        countdown(3, 2),
        countdown(3, 3),
    ]));
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    for _ in 0..3 {
        assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
    }
    assert_eq!(
        future.as_mut().poll(&mut context),
        Poll::Ready(vec![1, 2, 3])
    );
}
