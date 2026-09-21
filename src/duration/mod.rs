// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Parsing and formatting of durations as short human-readable strings (`1h30m`).
//!
//! The strings use the units `ms`, `s`, `m`, `h`, `d` and `w`. Values are [`std::time::Duration`].

mod error;
mod format;
mod parse;

pub use error::ParseDurationError;
pub use format::format;
pub use parse::parse;
