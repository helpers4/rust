// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// The byte order of a multi-byte integer.
///
/// # Examples
///
/// ```
/// use helpers4::bytes::{read_u16, Endian};
///
/// let bytes = [0x12, 0x34];
/// assert_eq!(read_u16(&bytes, 0, Endian::Big), Some(0x1234));
/// assert_eq!(read_u16(&bytes, 0, Endian::Little), Some(0x3412));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endian {
    /// Most significant byte first (network order).
    Big,
    /// Least significant byte first.
    Little,
}
