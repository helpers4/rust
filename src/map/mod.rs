// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Helpers for [`HashMap`](std::collections::HashMap) that the standard library does not provide.
//!
//! Inputs are borrowed and results are new maps: nothing is mutated.

mod map_values;
mod omit;
mod pick;

pub use map_values::map_values;
pub use omit::omit;
pub use pick::pick;
