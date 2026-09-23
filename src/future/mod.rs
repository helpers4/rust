// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Runtime-neutral helpers for `Future`s, built on the standard library only.
//!
//! No reactor, timer or thread pool is involved, so none of it depends on tokio, async-std or any other
//! runtime: `block_on` runs a future on the current thread, `join` and `join_all` combine futures on
//! one task, and `now_or_never` and `yield_now` are the small building blocks around them.

mod block_on;
mod join;
mod join_all;
mod now_or_never;
mod yield_now;

#[cfg(test)]
#[path = "_countdown.test.rs"]
mod countdown;

pub use block_on::block_on;
pub use join::join;
pub use join_all::join_all;
pub use now_or_never::now_or_never;
pub use yield_now::yield_now;
