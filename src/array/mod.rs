// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Helpers for slices and `Vec`s that the standard library does not provide.
//!
//! Inputs are borrowed slices and results are new `Vec`s: nothing is mutated. Membership-based
//! helpers require `Eq + Hash`.

mod difference;
mod intersection;
mod unique;
mod unique_by;

pub use difference::difference;
pub use intersection::intersection;
pub use unique::unique;
pub use unique_by::unique_by;
