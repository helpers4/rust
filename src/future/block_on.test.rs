// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

use crate::future::countdown::countdown;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

#[test]
fn returns_the_output_of_a_ready_future() {
    assert_eq!(block_on(countdown(0, 7)), 7);
}

#[test]
fn keeps_polling_a_pending_future_until_it_is_ready() {
    assert_eq!(block_on(countdown(5, 9)), 9);
}

#[test]
fn runs_an_async_block() {
    assert_eq!(block_on(async { countdown(2, 3).await + 1 }), 4);
}

/// Ready once another thread sets the flag and wakes the stored waker.
struct Signal {
    state: Arc<Mutex<(bool, Option<Waker>)>>,
}

impl Future for Signal {
    type Output = u32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<u32> {
        let mut state = self.state.lock().unwrap();
        if state.0 {
            Poll::Ready(11)
        } else {
            state.1 = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[test]
fn sleeps_until_another_thread_wakes_it() {
    let state = Arc::new(Mutex::new((false, None::<Waker>)));
    let remote = Arc::clone(&state);
    let worker = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(30));
        let mut state = remote.lock().unwrap();
        state.0 = true;
        if let Some(waker) = state.1.take() {
            waker.wake();
        }
    });
    assert_eq!(block_on(Signal { state }), 11);
    worker.join().unwrap();
}
