// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Helpers around functions: composition, memoization, retrying and rate limiting.
//!
//! Nothing here sleeps or reads a clock. `retry` and `backoff` leave the waiting to the caller, and
//! `TokenBucket` takes the current time as an argument, so all of it is deterministic to test.

mod backoff;
mod compose;
mod memoize;
mod pipe;
mod retry;
mod token_bucket;

pub use backoff::backoff;
pub use compose::compose;
pub use memoize::Memoize;
pub use pipe::pipe;
pub use retry::retry;
pub use token_bucket::TokenBucket;
