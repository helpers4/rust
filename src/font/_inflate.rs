// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! zlib decompression (RFC 1950 and 1951) for WOFF tables, a port of the idea of zlib's `puff`.
//! Not re-exported.

const LENGTH_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LENGTH_EXTRA: [u32; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DISTANCE_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DISTANCE_EXTRA: [u32; 30] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];
/// The order the code length code lengths are stored in.
const CODE_LENGTH_ORDER: [usize; 19] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

const MAX_BITS: usize = 15;

struct Bits<'a> {
    data: &'a [u8],
    position: usize,
    buffer: u32,
    count: u32,
}

impl<'a> Bits<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self {
            data,
            position: 0,
            buffer: 0,
            count: 0,
        }
    }

    /// The next `need` bits (at most 16), least significant first.
    fn bits(&mut self, need: u32) -> Option<u32> {
        let mut value = self.buffer;
        while self.count < need {
            value |= u32::from(*self.data.get(self.position)?) << self.count;
            self.position += 1;
            self.count += 8;
        }
        self.buffer = value >> need;
        self.count -= need;
        Some(value & ((1 << need) - 1))
    }

    /// Drops the bits left in the current byte and reads `len` whole bytes.
    fn aligned_bytes(&mut self, len: usize) -> Option<&'a [u8]> {
        self.buffer = 0;
        self.count = 0;
        let bytes = self
            .data
            .get(self.position..self.position.saturating_add(len))?;
        self.position += len;
        Some(bytes)
    }
}

/// A canonical Huffman code: how many codes there are of each length, and the symbols in code
/// order.
struct Huffman {
    count: [u16; MAX_BITS + 1],
    symbol: Vec<u16>,
    /// `false` when the lengths over-subscribe the code space: no symbol can be decoded.
    valid: bool,
}

impl Huffman {
    /// Builds the code from the length of each symbol's code (`0` for an unused symbol, at most
    /// 15). An incomplete code is accepted: a code that is not assigned is an error when it is
    /// read.
    #[allow(clippy::cast_possible_truncation)] // at most 320 symbols
    fn new(lengths: &[u8]) -> Self {
        let mut count = [0u16; MAX_BITS + 1];
        for &length in lengths {
            count[usize::from(length)] += 1;
        }
        let mut left: i32 = 1;
        for &in_length in &count[1..] {
            left = (left << 1) - i32::from(in_length);
        }
        let mut offsets = [0u16; MAX_BITS + 2];
        for length in 1..=MAX_BITS {
            offsets[length + 1] = offsets[length] + count[length];
        }
        let mut symbol = vec![0u16; usize::from(offsets[MAX_BITS + 1])];
        for (index, &length) in lengths.iter().enumerate() {
            if length != 0 {
                let slot = &mut offsets[usize::from(length)];
                symbol[usize::from(*slot)] = index as u16;
                *slot += 1;
            }
        }
        // `left` goes negative as soon as a length has more codes than the code space holds, and
        // stays negative from then on.
        Self {
            count,
            symbol,
            valid: left >= 0,
        }
    }

    fn decode(&self, bits: &mut Bits<'_>) -> Option<u16> {
        if !self.valid {
            return None;
        }
        let (mut code, mut first, mut index) = (0u32, 0u32, 0u32);
        for length in 1..=MAX_BITS {
            code |= bits.bits(1)?;
            let count = u32::from(self.count[length]);
            if code < first + count {
                return self.symbol.get((index + code - first) as usize).copied();
            }
            index += count;
            first = (first + count) << 1;
            code <<= 1;
        }
        None
    }
}

fn fixed_codes() -> (Huffman, Huffman) {
    let mut lengths = [8u8; 288];
    lengths[144..256].fill(9);
    lengths[256..280].fill(7);
    (Huffman::new(&lengths), Huffman::new(&[5u8; 30]))
}

#[allow(clippy::cast_possible_truncation)] // code lengths are at most 15
fn dynamic_codes(bits: &mut Bits<'_>) -> Option<(Huffman, Huffman)> {
    // The three counts: 5, 5 and 4 bits.
    let counts = bits.bits(14)? as usize;
    let literals = (counts & 31) + 257;
    let distances = ((counts >> 5) & 31) + 1;
    let code_lengths = (counts >> 10) + 4;
    if literals > 286 || distances > 30 {
        return None;
    }
    let mut lengths = [0u8; 19];
    for &slot in CODE_LENGTH_ORDER.iter().take(code_lengths) {
        lengths[slot] = bits.bits(3)? as u8;
    }
    let code_code = Huffman::new(&lengths);

    let total = literals + distances;
    let mut all: Vec<u8> = Vec::with_capacity(total);
    while all.len() < total {
        let symbol = code_code.decode(bits)?;
        let (value, repeat) = match symbol {
            0..=15 => (symbol as u8, 1),
            16 => (*all.last()?, 3 + bits.bits(2)? as usize),
            17 => (0, 3 + bits.bits(3)? as usize),
            _ => (0, 11 + bits.bits(7)? as usize),
        };
        if all.len() + repeat > total {
            return None;
        }
        all.resize(all.len() + repeat, value);
    }
    // Without an end-of-block code the block could never end.
    if all[256] == 0 {
        return None;
    }
    Some((
        Huffman::new(&all[..literals]),
        Huffman::new(&all[literals..]),
    ))
}

#[allow(clippy::cast_possible_truncation)] // a literal is below 256
fn codes(
    bits: &mut Bits<'_>,
    out: &mut Vec<u8>,
    limit: usize,
    literals: &Huffman,
    distances: &Huffman,
) -> Option<()> {
    loop {
        let symbol = usize::from(literals.decode(bits)?);
        match symbol {
            0..=255 => {
                if out.len() >= limit {
                    return None;
                }
                out.push(symbol as u8);
            }
            256 => return Some(()),
            _ => {
                let slot = symbol - 257;
                let length =
                    usize::from(*LENGTH_BASE.get(slot)?) + bits.bits(LENGTH_EXTRA[slot])? as usize;
                let slot = usize::from(distances.decode(bits)?);
                let distance =
                    usize::from(DISTANCE_BASE[slot]) + bits.bits(DISTANCE_EXTRA[slot])? as usize;
                if distance > out.len() || out.len() + length > limit {
                    return None;
                }
                for _ in 0..length {
                    out.push(out[out.len() - distance]);
                }
            }
        }
    }
}

fn stored(bits: &mut Bits<'_>, out: &mut Vec<u8>, limit: usize) -> Option<()> {
    let header = bits.aligned_bytes(4)?;
    let len = usize::from(u16::from_le_bytes([header[0], header[1]]));
    if header[2..4] != [!header[0], !header[1]] || out.len() + len > limit {
        return None;
    }
    out.extend_from_slice(bits.aligned_bytes(len)?);
    Some(())
}

/// Decompresses a zlib stream to exactly `size` bytes. `None` for anything else: a bad header,
/// invalid or truncated data, more or fewer bytes than announced. The checksum is not verified.
pub(crate) fn zlib_decompress(data: &[u8], size: usize) -> Option<Vec<u8>> {
    let (cmf, flg) = (*data.first()?, *data.get(1)?);
    if cmf & 0x0f != 8 || (u16::from(cmf) * 256 + u16::from(flg)) % 31 != 0 || flg & 0x20 != 0 {
        return None;
    }
    let mut bits = Bits::new(&data[2..]);
    let mut out = Vec::new();
    loop {
        let last = bits.bits(1)?;
        match bits.bits(2)? {
            0 => stored(&mut bits, &mut out, size)?,
            1 => {
                let (literals, distances) = fixed_codes();
                codes(&mut bits, &mut out, size, &literals, &distances)?;
            }
            2 => {
                let (literals, distances) = dynamic_codes(&mut bits)?;
                codes(&mut bits, &mut out, size, &literals, &distances)?;
            }
            _ => return None,
        }
        if last == 1 {
            return (out.len() == size).then_some(out);
        }
    }
}

#[cfg(test)]
#[path = "_inflate.test.rs"]
mod tests;
