// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! ANSI escape sequences: remove them from captured output and build styled text.
//!
//! `strip`, `contains` and `visible_len` deal with text that already has escape sequences (command
//! output, logs); [`Style`] and [`Color`] build the sequences for colors and attributes. Nothing here
//! decides whether a terminal supports colors: that stays with the caller.

mod color;
mod contains;
mod strip;
mod style;
mod visible_len;

pub use color::Color;
pub use contains::contains;
pub use strip::strip;
pub use style::Style;
pub use visible_len::visible_len;
