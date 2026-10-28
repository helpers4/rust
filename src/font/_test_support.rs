// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Builders of tiny fonts for the unit tests. Not part of the crate.

#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]

pub(super) type Table = ([u8; 4], Vec<u8>);

/// A `name` record: platform, encoding, language, name id, and its (already encoded) string.
pub(super) type NameRecord = (u16, u16, u16, u16, Vec<u8>);

pub(super) fn utf16(text: &str) -> Vec<u8> {
    text.encode_utf16().flat_map(u16::to_be_bytes).collect()
}

/// A Windows, English (US) record.
pub(super) fn windows(id: u16, text: &str) -> NameRecord {
    (3, 1, 0x0409, id, utf16(text))
}

pub(super) fn name_table(records: &[NameRecord]) -> Vec<u8> {
    let storage_start = 6 + 12 * records.len();
    let mut out = Vec::new();
    out.extend(0u16.to_be_bytes());
    out.extend((records.len() as u16).to_be_bytes());
    out.extend((storage_start as u16).to_be_bytes());
    let mut storage: Vec<u8> = Vec::new();
    for (platform, encoding, language, id, text) in records {
        for field in [
            *platform,
            *encoding,
            *language,
            *id,
            text.len() as u16,
            storage.len() as u16,
        ] {
            out.extend(field.to_be_bytes());
        }
        storage.extend(text);
    }
    out.extend(storage);
    out
}

pub(super) fn os2_table(weight: u16, fs_selection: u16) -> Vec<u8> {
    let mut out = vec![0u8; 78];
    out[4..6].copy_from_slice(&weight.to_be_bytes());
    out[62..64].copy_from_slice(&fs_selection.to_be_bytes());
    out
}

pub(super) fn head_table(mac_style: u16) -> Vec<u8> {
    let mut out = vec![0u8; 54];
    out[44..46].copy_from_slice(&mac_style.to_be_bytes());
    out
}

fn fixed(value: f64) -> [u8; 4] {
    ((value * 65536.0) as i32).to_be_bytes()
}

/// An `fvar` table with axes `(tag, min, default, max)`.
pub(super) fn fvar(axes: &[(&[u8; 4], f64, f64, f64)]) -> Vec<u8> {
    let mut out = Vec::new();
    for field in [1u16, 0, 16, 2, axes.len() as u16, 20, 0, 0] {
        out.extend(field.to_be_bytes());
    }
    for (tag, min, default, max) in axes {
        out.extend(*tag);
        out.extend(fixed(*min));
        out.extend(fixed(*default));
        out.extend(fixed(*max));
        out.extend([0u8; 4]);
    }
    out
}

/// The tables of a typical font: family, style and PostScript name; weight and style flags.
pub(super) fn typical_tables(
    family: &str,
    style: &str,
    weight: u16,
    fs_selection: u16,
) -> Vec<Table> {
    vec![
        (
            *b"name",
            name_table(&[
                windows(1, family),
                windows(2, style),
                windows(
                    6,
                    &format!("{}-{}", family.replace(' ', ""), style.replace(' ', "")),
                ),
            ]),
        ),
        (*b"OS/2", os2_table(weight, fs_selection)),
        (*b"head", head_table(0)),
    ]
}

/// An sfnt file (`00 01 00 00`) holding `tables`.
pub(super) fn sfnt(tables: &[Table]) -> Vec<u8> {
    sfnt_at(tables, 0, [0, 1, 0, 0])
}

/// An sfnt whose table directory starts at `start` (the bytes before are zeros).
pub(super) fn sfnt_at(tables: &[Table], start: usize, version: [u8; 4]) -> Vec<u8> {
    let directory = start + 12 + 16 * tables.len();
    let mut out = vec![0u8; start];
    out.extend(version);
    out.extend((tables.len() as u16).to_be_bytes());
    out.extend([0u8; 6]);
    let mut data = Vec::new();
    for (tag, bytes) in tables {
        out.extend(tag);
        out.extend([0u8; 4]);
        out.extend(((directory + data.len()) as u32).to_be_bytes());
        out.extend((bytes.len() as u32).to_be_bytes());
        data.extend(bytes);
        while data.len() % 4 != 0 {
            data.push(0);
        }
    }
    out.extend(data);
    out
}

/// A TrueType collection whose first font holds `tables`.
pub(super) fn collection(tables: &[Table]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend(b"ttcf");
    out.extend(0x0001_0000u32.to_be_bytes());
    out.extend(1u32.to_be_bytes());
    out.extend(16u32.to_be_bytes());
    // The offsets in the table records are from the start of the file: build the font at 16.
    out.extend(sfnt_at(tables, 16, [0, 1, 0, 0])[16..].iter());
    out
}

/// A WOFF file whose tables are stored as they are (`compLength == origLength`).
pub(super) fn woff(tables: &[Table]) -> Vec<u8> {
    let directory = 44 + 20 * tables.len();
    let mut out = Vec::new();
    out.extend(b"wOFF");
    out.extend([0, 1, 0, 0]);
    out.extend([0u8; 4]);
    out.extend((tables.len() as u16).to_be_bytes());
    out.extend([0u8; 30]);
    let mut data = Vec::new();
    for (tag, bytes) in tables {
        out.extend(tag);
        out.extend(((directory + data.len()) as u32).to_be_bytes());
        out.extend((bytes.len() as u32).to_be_bytes());
        out.extend((bytes.len() as u32).to_be_bytes());
        out.extend([0u8; 4]);
        data.extend(bytes);
        while data.len() % 4 != 0 {
            data.push(0);
        }
    }
    out.extend(data);
    out
}
