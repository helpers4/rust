// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! The tables of a font file that [`read_font`](super::read_font) reads, found in a TrueType,
//! OpenType or collection file (first font) or in a WOFF file. Not re-exported.

use super::_woff;
use super::FontError;
use std::borrow::Cow;

/// The tables [`read_font`](super::read_font) needs: names, weight and style, `wght` axis.
const WANTED: [[u8; 4]; 4] = [*b"name", *b"OS/2", *b"head", *b"fvar"];

pub(crate) fn be16(bytes: &[u8], at: usize) -> u16 {
    u16::from_be_bytes([bytes[at], bytes[at + 1]])
}

pub(crate) fn be32(bytes: &[u8], at: usize) -> u32 {
    u32::from_be_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

pub(crate) fn is_wanted(tag: [u8; 4]) -> bool {
    WANTED.contains(&tag)
}

/// The wanted tables of a font, decompressed when the file was WOFF.
pub(crate) struct Tables<'a> {
    pub(crate) entries: Vec<([u8; 4], Cow<'a, [u8]>)>,
}

impl Tables<'_> {
    pub(crate) fn get(&self, tag: [u8; 4]) -> Option<&[u8]> {
        self.entries
            .iter()
            .find(|(known, _)| *known == tag)
            .map(|(_, data)| &**data)
    }
}

/// The tables of the font in `bytes`.
pub(crate) fn load(bytes: &[u8]) -> Result<Tables<'_>, FontError> {
    let magic = bytes.get(..4).ok_or(FontError::Truncated)?;
    match magic {
        [0, 1, 0, 0] | b"OTTO" | b"true" => from_sfnt(bytes, 0),
        b"ttcf" => {
            let header = bytes.get(..16).ok_or(FontError::Truncated)?;
            from_sfnt(bytes, be32(header, 12) as usize)
        }
        b"wOFF" => _woff::load(bytes),
        b"wOF2" => Err(FontError::UnsupportedWoff2),
        _ => Err(FontError::UnknownFormat),
    }
}

/// The tables of the sfnt font whose table directory starts at `start`.
fn from_sfnt(bytes: &[u8], start: usize) -> Result<Tables<'_>, FontError> {
    let header = bytes
        .get(start..start.saturating_add(12))
        .ok_or(FontError::Truncated)?;
    let count = usize::from(be16(header, 4));
    let mut entries = Vec::new();
    for index in 0..count {
        let at = start + 12 + 16 * index;
        let record = bytes.get(at..at + 16).ok_or(FontError::Truncated)?;
        let tag = [record[0], record[1], record[2], record[3]];
        if is_wanted(tag) {
            let offset = be32(record, 8) as usize;
            let length = be32(record, 12) as usize;
            let table = bytes
                .get(offset..offset.saturating_add(length))
                .ok_or(FontError::Truncated)?;
            entries.push((tag, Cow::Borrowed(table)));
        }
    }
    Ok(Tables { entries })
}

#[cfg(test)]
#[path = "_sfnt.test.rs"]
mod tests;
