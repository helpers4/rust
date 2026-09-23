// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! A hand-written future for the tests: pending a given number of times, then ready.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

pub(crate) struct Countdown {
    pub(crate) remaining: u32,
    pub(crate) value: u32,
}

/// A future that is pending `remaining` times (waking itself each time), then ready with `value`.
pub(crate) fn countdown(remaining: u32, value: u32) -> Countdown {
    Countdown { remaining, value }
}

impl Future for Countdown {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        if self.remaining == 0 {
            Poll::Ready(self.value)
        } else {
            self.remaining -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
