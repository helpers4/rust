// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Helpers for slices and `Vec`s that the standard library does not provide.
//!
//! Inputs are borrowed slices and results are new `Vec`s: nothing is mutated. Membership-based
//! helpers require `Eq + Hash`.

mod cartesian_product;
mod count_by;
mod difference;
mod duplicates;
mod equals_unordered;
mod group_by;
mod interleave;
mod intersection;
mod intersects;
mod key_by;
mod symmetric_difference;
mod unique;
mod unique_by;

pub use cartesian_product::cartesian_product;
pub use count_by::count_by;
pub use difference::difference;
pub use duplicates::duplicates;
pub use equals_unordered::equals_unordered;
pub use group_by::group_by;
pub use interleave::interleave;
pub use intersection::intersection;
pub use intersects::intersects;
pub use key_by::key_by;
pub use symmetric_difference::symmetric_difference;
pub use unique::unique;
pub use unique_by::unique_by;
