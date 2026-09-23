// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::future::Future;
use std::task::Poll;

/// Runs any number of futures concurrently and completes with all their outputs, in the order the
/// futures were given.
///
/// Every future still pending is polled on each wake-up, so they progress together on one task. An
/// empty input completes at once with an empty `Vec`. Like [`join`](super::join) it needs no
/// runtime.
///
/// # Arguments
///
/// - `futures` - The futures to run, all of the same type (box them to mix types).
///
/// # Returns
///
/// A future for the outputs, in input order.
///
/// # Examples
///
/// ```
/// use helpers4::future::{block_on, join_all};
///
/// let squares = block_on(join_all((1..=4).map(|n| async move { n * n })));
/// assert_eq!(squares, vec![1, 4, 9, 16]);
/// ```
pub fn join_all<I>(futures: I) -> impl Future<Output = Vec<<I::Item as Future>::Output>>
where
    I: IntoIterator,
    I::Item: Future,
{
    let mut pending: Vec<_> = futures.into_iter().map(Box::pin).collect();
    let mut outputs: Vec<Option<<I::Item as Future>::Output>> =
        pending.iter().map(|_| None).collect();
    std::future::poll_fn(move |context| {
        for (future, output) in pending.iter_mut().zip(outputs.iter_mut()) {
            if output.is_none() {
                if let Poll::Ready(value) = future.as_mut().poll(context) {
                    *output = Some(value);
                }
            }
        }
        if outputs.iter().all(Option::is_some) {
            Poll::Ready(outputs.iter_mut().filter_map(Option::take).collect())
        } else {
            Poll::Pending
        }
    })
}

#[cfg(test)]
#[path = "join_all.test.rs"]
mod tests;

#[cfg(test)]
#[path = "join_all.spec.rs"]
mod spec;
