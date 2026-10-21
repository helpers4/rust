// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{Endian, read_u16, read_u32, read_u64, zigzag_decode};

/// A cursor over a byte slice, for decoding binary formats: fixed-size integers and floats,
/// LEB128 varints (signed ones zigzag-encoded), byte runs and NUL-terminated strings.
///
/// Every `read_*` method returns `None` when the data is truncated or invalid, and then leaves
/// the position where it was, so [`position`](Self::position) is the offset of the read that
/// failed: enough to build a precise error in the caller's own error type.
///
/// # Examples
///
/// ```
/// use helpers4::bytes::{ByteReader, Endian};
///
/// let data = [0x2a, 0xac, 0x02, b'h', b'i', 0, 0x01, 0x00];
/// let mut reader = ByteReader::new(&data);
/// assert_eq!(reader.read_u8(), Some(42));
/// assert_eq!(reader.read_var_u32(), Some(300));
/// assert_eq!(reader.read_cstr(), Some("hi"));
/// assert_eq!(reader.read_u16(Endian::Little), Some(1));
/// assert_eq!(reader.read_u8(), None); // end of data
/// assert_eq!(reader.position(), 8);
/// ```
#[derive(Debug, Clone)]
pub struct ByteReader<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> ByteReader<'a> {
    /// Starts reading `data` from its first byte.
    ///
    /// # Arguments
    ///
    /// - `data` - The bytes to decode.
    ///
    /// # Returns
    ///
    /// A reader at position `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::bytes::ByteReader;
    ///
    /// assert_eq!(ByteReader::new(&[1, 2, 3]).remaining(), 3);
    /// ```
    #[must_use]
    pub const fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    /// The offset of the next byte to read.
    ///
    /// # Returns
    ///
    /// The number of bytes consumed so far.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::bytes::ByteReader;
    ///
    /// let mut reader = ByteReader::new(&[1, 2, 3]);
    /// let _ = reader.read_u8();
    /// assert_eq!(reader.position(), 1);
    /// ```
    #[must_use]
    pub const fn position(&self) -> usize {
        self.position
    }

    /// The number of bytes left to read.
    ///
    /// # Returns
    ///
    /// `0` at the end of the data.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::bytes::ByteReader;
    ///
    /// let mut reader = ByteReader::new(&[1, 2, 3]);
    /// let _ = reader.read_bytes(2);
    /// assert_eq!(reader.remaining(), 1);
    /// ```
    #[must_use]
    pub const fn remaining(&self) -> usize {
        self.data.len() - self.position
    }

    /// Reads one byte.
    ///
    /// # Returns
    ///
    /// The byte, or `None` at the end of the data.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::bytes::ByteReader;
    ///
    /// let mut reader = ByteReader::new(&[7]);
    /// assert_eq!(reader.read_u8(), Some(7));
    /// assert_eq!(reader.read_u8(), None);
    /// ```
    pub fn read_u8(&mut self) -> Option<u8> {
        let byte = *self.data.get(self.position)?;
        self.position += 1;
        Some(byte)
    }

    /// Reads `len` bytes, borrowed from the data (no copy).
    ///
    /// # Arguments
    ///
    /// - `len` - The number of bytes.
    ///
    /// # Returns
    ///
    /// The bytes, or `None` when fewer than `len` remain.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::bytes::ByteReader;
    ///
    /// let mut reader = ByteReader::new(b"abc");
    /// assert_eq!(reader.read_bytes(2), Some(&b"ab"[..]));
    /// assert_eq!(reader.read_bytes(2), None);
    /// ```
    pub fn read_bytes(&mut self, len: usize) -> Option<&'a [u8]> {
        let end = self.position.checked_add(len)?;
        let bytes = self.data.get(self.position..end)?;
        self.position = end;
        Some(bytes)
    }

    /// Reads a `u16` in the given byte order.
    ///
    /// # Arguments
    ///
    /// - `endian` - The byte order of the value.
    ///
    /// # Returns
    ///
    /// The value, or `None` when fewer than 2 bytes remain.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::bytes::{ByteReader, Endian};
    ///
    /// assert_eq!(ByteReader::new(&[0x12, 0x34]).read_u16(Endian::Big), Some(0x1234));
    /// ```
    pub fn read_u16(&mut self, endian: Endian) -> Option<u16> {
        let value = read_u16(self.data, self.position, endian)?;
        self.position += 2;
        Some(value)
    }

    /// Reads a `u32` in the given byte order.
    ///
    /// # Arguments
    ///
    /// - `endian` - The byte order of the value.
    ///
    /// # Returns
    ///
    /// The value, or `None` when fewer than 4 bytes remain.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::bytes::{ByteReader, Endian};
    ///
    /// assert_eq!(ByteReader::new(&[1, 0, 0, 0]).read_u32(Endian::Little), Some(1));
    /// ```
    pub fn read_u32(&mut self, endian: Endian) -> Option<u32> {
        let value = read_u32(self.data, self.position, endian)?;
        self.position += 4;
        Some(value)
    }

    /// Reads a `u64` in the given byte order.
    ///
    /// # Arguments
    ///
    /// - `endian` - The byte order of the value.
    ///
    /// # Returns
    ///
    /// The value, or `None` when fewer than 8 bytes remain.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::bytes::{ByteReader, Endian};
    ///
    /// assert_eq!(ByteReader::new(&[0, 0, 0, 0, 0, 0, 0, 9]).read_u64(Endian::Big), Some(9));
    /// ```
    pub fn read_u64(&mut self, endian: Endian) -> Option<u64> {
        let value = read_u64(self.data, self.position, endian)?;
        self.position += 8;
        Some(value)
    }

    /// Reads an `i16` (two's complement) in the given byte order.
    ///
    /// # Arguments
    ///
    /// - `endian` - The byte order of the value.
    ///
    /// # Returns
    ///
    /// The value, or `None` when fewer than 2 bytes remain.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::bytes::{ByteReader, Endian};
    ///
    /// assert_eq!(ByteReader::new(&[0xff, 0xfe]).read_i16(Endian::Big), Some(-2));
    /// ```
    pub fn read_i16(&mut self, endian: Endian) -> Option<i16> {
        self.read_u16(endian)
            .map(|v| i16::from_ne_bytes(v.to_ne_bytes()))
    }

    /// Reads an `i32` (two's complement) in the given byte order.
    ///
    /// # Arguments
    ///
    /// - `endian` - The byte order of the value.
    ///
    /// # Returns
    ///
    /// The value, or `None` when fewer than 4 bytes remain.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::bytes::{ByteReader, Endian};
    ///
    /// assert_eq!(ByteReader::new(&[0xff; 4]).read_i32(Endian::Little), Some(-1));
    /// ```
    pub fn read_i32(&mut self, endian: Endian) -> Option<i32> {
        self.read_u32(endian)
            .map(|v| i32::from_ne_bytes(v.to_ne_bytes()))
    }

    /// Reads an `i64` (two's complement) in the given byte order.
    ///
    /// # Arguments
    ///
    /// - `endian` - The byte order of the value.
    ///
    /// # Returns
    ///
    /// The value, or `None` when fewer than 8 bytes remain.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::bytes::{ByteReader, Endian};
    ///
    /// assert_eq!(ByteReader::new(&[0xff; 8]).read_i64(Endian::Big), Some(-1));
    /// ```
    pub fn read_i64(&mut self, endian: Endian) -> Option<i64> {
        self.read_u64(endian)
            .map(|v| i64::from_ne_bytes(v.to_ne_bytes()))
    }

    /// Reads an IEEE 754 `f32` in the given byte order.
    ///
    /// # Arguments
    ///
    /// - `endian` - The byte order of the value.
    ///
    /// # Returns
    ///
    /// The value, or `None` when fewer than 4 bytes remain.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::bytes::{ByteReader, Endian};
    ///
    /// let bytes = 1.5f32.to_le_bytes();
    /// assert_eq!(ByteReader::new(&bytes).read_f32(Endian::Little), Some(1.5));
    /// ```
    pub fn read_f32(&mut self, endian: Endian) -> Option<f32> {
        self.read_u32(endian).map(f32::from_bits)
    }

    /// Reads an IEEE 754 `f64` in the given byte order.
    ///
    /// # Arguments
    ///
    /// - `endian` - The byte order of the value.
    ///
    /// # Returns
    ///
    /// The value, or `None` when fewer than 8 bytes remain.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::bytes::{ByteReader, Endian};
    ///
    /// let bytes = (-0.25f64).to_be_bytes();
    /// assert_eq!(ByteReader::new(&bytes).read_f64(Endian::Big), Some(-0.25));
    /// ```
    pub fn read_f64(&mut self, endian: Endian) -> Option<f64> {
        self.read_u64(endian).map(f64::from_bits)
    }

    /// Reads an unsigned LEB128 varint of at most 32 bits: 7 bits per byte, least significant
    /// first, the high bit set on every byte but the last.
    ///
    /// Overlong encodings (`0x80 0x00` for `0`) are accepted, as most decoders do.
    ///
    /// # Returns
    ///
    /// The value, or `None` when the data ends inside the varint or the value needs more than
    /// 32 bits (a 5th byte above `0x0f`, or a 6th byte).
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::bytes::ByteReader;
    ///
    /// assert_eq!(ByteReader::new(&[0xac, 0x02]).read_var_u32(), Some(300));
    /// assert_eq!(ByteReader::new(&[0xff, 0xff, 0xff, 0xff, 0x0f]).read_var_u32(), Some(u32::MAX));
    /// assert_eq!(ByteReader::new(&[0xff, 0xff, 0xff, 0xff, 0x1f]).read_var_u32(), None);
    /// ```
    pub fn read_var_u32(&mut self) -> Option<u32> {
        let mut value = 0u32;
        let end = self.leb128(32, |payload, shift| value |= u32::from(payload) << shift)?;
        self.position = end;
        Some(value)
    }

    /// Reads an unsigned LEB128 varint of at most 64 bits (up to 10 bytes), like
    /// [`read_var_u32`](Self::read_var_u32).
    ///
    /// # Returns
    ///
    /// The value, or `None` when the data ends inside the varint or the value needs more than
    /// 64 bits (a 10th byte above `0x01`, or an 11th byte).
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::bytes::ByteReader;
    ///
    /// let max = [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01];
    /// assert_eq!(ByteReader::new(&max).read_var_u64(), Some(u64::MAX));
    /// ```
    pub fn read_var_u64(&mut self) -> Option<u64> {
        let mut value = 0u64;
        let end = self.leb128(64, |payload, shift| value |= u64::from(payload) << shift)?;
        self.position = end;
        Some(value)
    }

    /// Reads a signed varint: a zigzag-encoded (see [`zigzag_decode`]) LEB128 varint of at
    /// most 32 bits.
    ///
    /// # Returns
    ///
    /// The value, or `None` as for [`read_var_u32`](Self::read_var_u32).
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::bytes::ByteReader;
    ///
    /// assert_eq!(ByteReader::new(&[0x01]).read_var_i32(), Some(-1));
    /// assert_eq!(ByteReader::new(&[0x02]).read_var_i32(), Some(1));
    /// ```
    pub fn read_var_i32(&mut self) -> Option<i32> {
        let raw = self.read_var_u32()?;
        // 32-bit zigzag, same mapping as zigzag_decode, without a fallible narrowing.
        Some(
            i32::from_ne_bytes((raw >> 1).to_ne_bytes())
                ^ -i32::from_ne_bytes((raw & 1).to_ne_bytes()),
        )
    }

    /// Reads a signed varint: a zigzag-encoded (see [`zigzag_decode`]) LEB128 varint of at
    /// most 64 bits.
    ///
    /// # Returns
    ///
    /// The value, or `None` as for [`read_var_u64`](Self::read_var_u64).
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::bytes::ByteReader;
    ///
    /// assert_eq!(ByteReader::new(&[0x03]).read_var_i64(), Some(-2));
    /// ```
    pub fn read_var_i64(&mut self) -> Option<i64> {
        self.read_var_u64().map(zigzag_decode)
    }

    /// Reads a NUL-terminated UTF-8 string and skips the terminating `0`.
    ///
    /// # Returns
    ///
    /// The string without its terminator, borrowed from the data, or `None` when there is no
    /// `0` byte before the end or the bytes are not valid UTF-8.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::bytes::ByteReader;
    ///
    /// let mut reader = ByteReader::new(b"Frame 1\0\0");
    /// assert_eq!(reader.read_cstr(), Some("Frame 1"));
    /// assert_eq!(reader.read_cstr(), Some(""));
    /// assert_eq!(reader.read_cstr(), None);
    /// ```
    pub fn read_cstr(&mut self) -> Option<&'a str> {
        let rest = &self.data[self.position..];
        let len = rest.iter().position(|&byte| byte == 0)?;
        let text = std::str::from_utf8(&rest[..len]).ok()?;
        self.position += len + 1;
        Some(text)
    }

    /// Walks a LEB128 varint of at most `bits` bits from the current position without moving it,
    /// calling `add(payload, shift)` for each 7-bit group, and returns the offset just past it.
    fn leb128(&self, bits: u32, mut add: impl FnMut(u8, u32)) -> Option<usize> {
        let mut position = self.position;
        let mut shift = 0;
        loop {
            let byte = *self.data.get(position)?;
            position += 1;
            let payload = byte & 0x7f;
            // Each group holds 7 bits, except the last one, which only has room for the bits left:
            // a payload that does not fit overflows.
            let room = (bits - shift).min(7);
            if u32::from(payload) >= 1 << room {
                return None;
            }
            add(payload, shift);
            if byte & 0x80 == 0 {
                return Some(position);
            }
            shift += 7;
            if shift >= bits {
                return None;
            }
        }
    }
}

#[cfg(test)]
#[path = "byte_reader.test.rs"]
mod tests;

#[cfg(test)]
#[path = "byte_reader.spec.rs"]
mod spec;
