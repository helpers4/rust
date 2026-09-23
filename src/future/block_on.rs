// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::future::Future;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::thread::{self, Thread};

/// Wakes the thread that is blocked in [`block_on`].
struct ThreadWaker(Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}

/// Runs `future` to completion on the current thread and returns its output.
///
/// A minimal executor with no dependency: it polls the future, and while it is pending parks the
/// thread until the future's waker is called. There is no reactor, timer or thread pool, so it
/// suits futures that are purely computational or woken by another thread (channels, locks,
/// your own `Waker` users). A future that needs a specific runtime (tokio's sockets or timers,
/// for instance) must run in that runtime instead, and blocking inside one would deadlock it.
///
/// # Arguments
///
/// - `future` - The future to run.
///
/// # Returns
///
/// The output of `future`.
///
/// # Examples
///
/// ```
/// use helpers4::future::block_on;
///
/// let answer = block_on(async { 40 + 2 });
/// assert_eq!(answer, 42);
/// ```
pub fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = std::pin::pin!(future);
    let waker = Waker::from(Arc::new(ThreadWaker(thread::current())));
    let mut context = Context::from_waker(&waker);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => thread::park(),
        }
    }
}

#[cfg(test)]
#[path = "block_on.test.rs"]
mod tests;
