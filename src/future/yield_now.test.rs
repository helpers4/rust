// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

use crate::future::block_on;
use crate::future::now_or_never;

#[test]
fn is_pending_once_and_then_ready() {
    let mut future = Box::pin(yield_now());
    let mut context = std::task::Context::from_waker(std::task::Waker::noop());
    assert_eq!(future.as_mut().poll(&mut context), Poll::Pending);
    assert_eq!(future.as_mut().poll(&mut context), Poll::Ready(()));
}

#[test]
fn wakes_the_task_so_an_executor_resumes_it() {
    block_on(yield_now());
}

#[test]
fn is_not_ready_on_the_first_poll() {
    assert_eq!(now_or_never(yield_now()), None);
}
