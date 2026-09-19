// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! General-purpose helpers, one module per category.
//!
//! Module and function names may overlap across modules (`array::compact`
//! and `object::compact` are different functions); import through the module
//! path, e.g. `helpers4::string::capitalize`.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

#[cfg(feature = "array")]
pub mod array;

#[cfg(feature = "env")]
pub mod env;

#[cfg(feature = "hex")]
pub mod hex;

#[cfg(feature = "string")]
pub mod string;
