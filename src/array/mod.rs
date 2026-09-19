// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Helpers for slices and `Vec`s that the standard library does not provide.
//!
//! Inputs are borrowed slices and results are new `Vec`s: nothing is mutated. Membership-based
//! helpers require `Eq + Hash`.

mod unique;

pub use unique::unique;
