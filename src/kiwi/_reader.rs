// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! A cursor over Kiwi bytes: LEB128 varints, zigzag integers, Kiwi floats and NUL-terminated
//! strings, on `ByteReader`. Kiwi's 64-bit varints and floats have their own encoding, read
//! here. Not re-exported.

use super::KiwiError;
use crate::bytes::ByteReader;

pub(crate) struct Reader<'a> {
    bytes: ByteReader<'a>,
}

impl<'a> Reader<'a> {
    pub(crate) fn new(data: &'a [u8]) -> Self {
        Self {
            bytes: ByteReader::new(data),
        }
    }

    pub(crate) fn remaining(&self) -> usize {
        self.bytes.remaining()
    }

    /// The end of the data, where a read fell short.
    fn eof(&self) -> KiwiError {
        KiwiError::UnexpectedEof {
            index: self.bytes.position(),
        }
    }

    pub(crate) fn byte(&mut self) -> Result<u8, KiwiError> {
        self.bytes.read_u8().ok_or_else(|| self.eof())
    }

    pub(crate) fn take(&mut self, len: usize) -> Result<&'a [u8], KiwiError> {
        self.bytes.read_bytes(len).ok_or_else(|| self.eof())
    }

    pub(crate) fn var_uint(&mut self) -> Result<u32, KiwiError> {
        self.bytes.read_var_u32().ok_or_else(|| self.eof())
    }

    pub(crate) fn var_int(&mut self) -> Result<i32, KiwiError> {
        self.bytes.read_var_i32().ok_or_else(|| self.eof())
    }

    /// Kiwi's 64-bit varints are not LEB128: after eight groups of 7 bits, the ninth byte holds
    /// 8 full bits.
    pub(crate) fn var_uint64(&mut self) -> Result<u64, KiwiError> {
        let mut value = 0u64;
        for shift in (0..56).step_by(7) {
            let byte = self.byte()?;
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Ok(value | (u64::from(self.byte()?) << 56))
    }

    #[allow(clippy::cast_possible_wrap)] // zigzag: the decoded bits are reinterpreted
    pub(crate) fn var_int64(&mut self) -> Result<i64, KiwiError> {
        let v = self.var_uint64()?;
        Ok(((v >> 1) as i64) ^ -((v & 1) as i64))
    }

    /// Kiwi stores floats with the exponent moved to the low bits, and a single `0` byte for zero.
    pub(crate) fn var_float(&mut self) -> Result<f32, KiwiError> {
        let first = self.byte()?;
        if first == 0 {
            return Ok(0.0);
        }
        let rest = self.take(3)?;
        let bits = u32::from_le_bytes([first, rest[0], rest[1], rest[2]]);
        Ok(f32::from_bits(bits.rotate_left(23)))
    }

    pub(crate) fn string(&mut self) -> Result<String, KiwiError> {
        let start = self.bytes.position();
        self.bytes
            .read_cstr()
            .map(str::to_owned)
            .ok_or(KiwiError::InvalidString { index: start })
    }
}

#[cfg(test)]
#[path = "_reader.test.rs"]
mod tests;
