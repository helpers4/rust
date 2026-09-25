// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Byte/text encodings with typed errors: hexadecimal, and base64/base32 (RFC 4648).
//!
//! Hex decoding accepts either case and rejects anything that is not pairs of hex digits, so
//! trim whitespace and strip `0x` prefixes before calling it. base64/base32 decoding requires
//! `=` padding to a multiple of 4/8 characters and rejects it anywhere but the end.

mod _digits;
mod base32_decode;
mod base32_encode;
mod base32_error;
mod base64_decode;
mod base64_encode;
mod base64_error;
mod decode;
mod decode_array;
mod decode_to_slice;
mod encode;
mod encode_upper;
mod error;

pub use base32_decode::base32_decode;
pub use base32_encode::base32_encode;
pub use base32_error::Base32DecodeError;
pub use base64_decode::base64_decode;
pub use base64_encode::base64_encode;
pub use base64_error::Base64DecodeError;
pub use decode::decode;
pub use decode_array::decode_array;
pub use decode_to_slice::decode_to_slice;
pub use encode::encode;
pub use encode_upper::encode_upper;
pub use error::DecodeError;
