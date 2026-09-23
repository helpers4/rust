// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Helpers for [`HashSet`](std::collections::HashSet) that the standard library does not provide.
//!
//! The standard library already has the pairwise operations (`union`, `intersection`, `difference`, ...);
//! these cover what it lacks: combining any number of sets, toggling a member, a stable order, a
//! similarity score and the power set. Inputs are borrowed and results are new values.

mod intersection_all;
mod jaccard;
mod subsets;
mod to_sorted_vec;
mod toggle;
mod union_all;

pub use intersection_all::intersection_all;
pub use jaccard::jaccard;
pub use subsets::MAX_SUBSET_ITEMS;
pub use subsets::subsets;
pub use to_sorted_vec::to_sorted_vec;
pub use toggle::toggle;
pub use union_all::union_all;
