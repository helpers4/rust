// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Helpers for byte slices and sizes that the standard library does not provide.
//!
//! Reading integers at an offset without panicking, a byte-slice search, a constant-time comparison,
//! XOR, and human-readable sizes (`1.5 KiB`) in both directions.

mod constant_time_eq;
mod endian;
mod error;
mod find;
mod format_size;
mod parse_size;
mod read_u16;
mod read_u32;
mod read_u64;
mod xor;

pub use constant_time_eq::constant_time_eq;
pub use endian::Endian;
pub use error::ParseSizeError;
pub use find::find;
pub use format_size::format_size;
pub use parse_size::parse_size;
pub use read_u16::read_u16;
pub use read_u32::read_u32;
pub use read_u64::read_u64;
pub use xor::xor;
