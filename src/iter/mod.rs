// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Helpers for any `Iterator`, not only slices: work on a lazy, single-use or unbounded source.

mod chunk;
mod first_duplicate;
mod min_max;

pub use chunk::chunk;
pub use first_duplicate::first_duplicate;
pub use min_max::min_max;
