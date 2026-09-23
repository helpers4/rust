// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::future::Future;
use std::task::Poll;

/// Runs two futures concurrently and completes with both outputs, once both are done.
///
/// Both are polled on every wake-up, so they make progress together on one task; neither is
/// started twice or dropped early. It needs no runtime, so it works with [`block_on`](super::block_on)
/// or any executor.
///
/// # Arguments
///
/// - `a` - The first future.
/// - `b` - The second future.
///
/// # Returns
///
/// A future for `(output of a, output of b)`.
///
/// # Examples
///
/// ```
/// use helpers4::future::{block_on, join};
///
/// let (a, b) = block_on(join(async { 1 }, async { "two" }));
/// assert_eq!((a, b), (1, "two"));
/// ```
pub fn join<A: Future, B: Future>(a: A, b: B) -> impl Future<Output = (A::Output, B::Output)> {
    let mut a = Box::pin(a);
    let mut b = Box::pin(b);
    let mut output_a = None;
    let mut output_b = None;
    std::future::poll_fn(move |context| {
        if output_a.is_none() {
            if let Poll::Ready(value) = a.as_mut().poll(context) {
                output_a = Some(value);
            }
        }
        if output_b.is_none() {
            if let Poll::Ready(value) = b.as_mut().poll(context) {
                output_b = Some(value);
            }
        }
        match (output_a.take(), output_b.take()) {
            (Some(x), Some(y)) => Poll::Ready((x, y)),
            (x, y) => {
                output_a = x;
                output_b = y;
                Poll::Pending
            }
        }
    })
}

#[cfg(test)]
#[path = "join.test.rs"]
mod tests;
