// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::future::Future;
use std::task::Poll;

/// A future that gives other tasks a turn: it is pending once, then completes.
///
/// The first poll wakes the task straight away and returns `Pending`, so an executor that runs
/// many tasks can schedule the others before resuming this one. Call it inside a long
/// computation in an `async` block to keep it from monopolising its thread.
///
/// # Returns
///
/// A future that completes on its second poll.
///
/// # Examples
///
/// ```
/// use helpers4::future::{block_on, yield_now};
///
/// block_on(async {
///     yield_now().await;
/// });
/// ```
pub fn yield_now() -> impl Future<Output = ()> {
    let mut yielded = false;
    std::future::poll_fn(move |context| {
        if yielded {
            Poll::Ready(())
        } else {
            yielded = true;
            context.waker().wake_by_ref();
            Poll::Pending
        }
    })
}

#[cfg(test)]
#[path = "yield_now.test.rs"]
mod tests;
