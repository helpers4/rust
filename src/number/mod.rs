// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Numeric helpers that the standard library does not provide.
//!
//! Floating-point helpers never panic: an input with no meaningful answer (an empty slice, a
//! zero total, a `NaN`) gives `None`, and `NaN` or infinite values propagate as usual for `f64`.

mod gcd;
mod lcm;
mod lerp;
mod mean;
mod median;
mod percentage;
mod round_to;

pub use gcd::gcd;
pub use lcm::lcm;
pub use lerp::lerp;
pub use mean::mean;
pub use median::median;
pub use percentage::percentage;
pub use round_to::round_to;
