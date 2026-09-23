// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::future::Future;
use std::task::{Context, Poll, Waker};

/// Polls `future` exactly once and returns its output if it was already ready.
///
/// Useful to peek at a future that may have finished (a channel receive, a cached value) without
/// waiting for it. The future is dropped if it was not ready, so use it on futures you can
/// afford to abandon.
///
/// # Arguments
///
/// - `future` - The future to poll once.
///
/// # Returns
///
/// `Some(output)` when the first poll completed, `None` when it was still pending.
///
/// # Examples
///
/// ```
/// use helpers4::future::now_or_never;
///
/// assert_eq!(now_or_never(async { 1 + 1 }), Some(2));
/// assert_eq!(now_or_never(std::future::pending::<u32>()), None);
/// ```
pub fn now_or_never<F: Future>(future: F) -> Option<F::Output> {
    let mut future = std::pin::pin!(future);
    let mut context = Context::from_waker(Waker::noop());
    match future.as_mut().poll(&mut context) {
        Poll::Ready(output) => Some(output),
        Poll::Pending => None,
    }
}

#[cfg(test)]
#[path = "now_or_never.test.rs"]
mod tests;
