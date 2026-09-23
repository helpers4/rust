// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! File-system helpers: safe writes, missing-file-as-`None`, listing and path checks.
//!
//! `write_atomic` and `read_to_string_opt` cover the two things `std::fs` makes awkward, `walk` lists a
//! tree without following links, and `normalize` / `is_within` handle paths lexically (no I/O, so they
//! work for paths that do not exist, and never resolve symbolic links).

mod _parts;
mod is_within;
mod normalize;
mod read_to_string_opt;
mod walk;
mod write_atomic;

#[cfg(test)]
#[path = "_scratch.test.rs"]
mod scratch;

pub use is_within::is_within;
pub use normalize::normalize;
pub use read_to_string_opt::read_to_string_opt;
pub use walk::walk;
pub use write_atomic::write_atomic;
