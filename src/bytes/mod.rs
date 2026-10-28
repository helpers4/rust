// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Helpers for byte slices and sizes that the standard library does not provide.
//!
//! Reading integers at an offset without panicking, a [`ByteReader`] cursor for binary formats
//! (fixed-size values, LEB128 varints, zigzag, NUL-terminated strings), a byte-slice search, a
//! constant-time comparison, XOR, human-readable sizes (`1.5 KiB`) in both directions, and the
//! media type of an image from its first bytes or of a file from its extension.

mod byte_reader;
mod constant_time_eq;
mod endian;
mod error;
mod find;
mod format_size;
mod media_type_of;
mod media_type_of_path;
mod parse_size;
mod read_u16;
mod read_u32;
mod read_u64;
mod xor;
mod zigzag_decode;
mod zigzag_encode;

pub use byte_reader::ByteReader;
pub use constant_time_eq::constant_time_eq;
pub use endian::Endian;
pub use error::ParseSizeError;
pub use find::find;
pub use format_size::format_size;
pub use media_type_of::media_type_of;
pub use media_type_of_path::media_type_of_path;
pub use parse_size::parse_size;
pub use read_u16::read_u16;
pub use read_u32::read_u32;
pub use read_u64::read_u64;
pub use xor::xor;
pub use zigzag_decode::zigzag_decode;
pub use zigzag_encode::zigzag_encode;
