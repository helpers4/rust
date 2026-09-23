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

#[cfg(feature = "bytes")]
pub mod bytes;

#[cfg(feature = "cache")]
pub mod cache;

#[cfg(feature = "date")]
pub mod date;

#[cfg(feature = "duration")]
pub mod duration;

#[cfg(feature = "env")]
pub mod env;

#[cfg(feature = "fs")]
pub mod fs;

#[cfg(feature = "hex")]
pub mod hex;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "iter")]
pub mod iter;

#[cfg(feature = "map")]
pub mod map;

#[cfg(feature = "net")]
pub mod net;

#[cfg(feature = "number")]
pub mod number;

#[cfg(feature = "set")]
pub mod set;

#[cfg(feature = "string")]
pub mod string;

#[cfg(feature = "time")]
pub mod time;

#[cfg(feature = "validate")]
pub mod validate;
