// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Time helpers. Reading the clock is explicit and fallible: a clock set before 1970 is an error, not `0`.

mod _epoch;
mod error;
mod unix_now;
mod unix_now_millis;

pub use error::ClockError;
pub use unix_now::unix_now;
pub use unix_now_millis::unix_now_millis;
