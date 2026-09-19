// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Hexadecimal encoding and decoding with typed errors.
//!
//! Decoding accepts either case and rejects anything that is not pairs of hex digits, so trim
//! whitespace and strip `0x` prefixes before calling it.

mod _digits;
mod decode;
mod decode_array;
mod decode_to_slice;
mod encode;
mod encode_upper;
mod error;

pub use decode::decode;
pub use decode_array::decode_array;
pub use decode_to_slice::decode_to_slice;
pub use encode::encode;
pub use encode_upper::encode_upper;
pub use error::DecodeError;
