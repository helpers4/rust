// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn riff(tag: &str) -> Vec<u8> {
    [b"RIFF\0\0\0\0".as_slice(), tag.as_bytes()].concat()
}

fn ftyp(brand: &str) -> Vec<u8> {
    [b"\0\0\0\x1cftyp".as_slice(), brand.as_bytes()].concat()
}

#[test]
fn recognizes_the_binary_formats() {
    assert_eq!(
        media_type_of(&[0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a, 0]),
        Some("image/png")
    );
    assert_eq!(media_type_of(&[0xff, 0xd8, 0xff, 0xe0]), Some("image/jpeg"));
    assert_eq!(media_type_of(b"GIF87a..."), Some("image/gif"));
    assert_eq!(media_type_of(b"GIF89a..."), Some("image/gif"));
    assert_eq!(media_type_of(&riff("WEBPVP8 ")), Some("image/webp"));
    assert_eq!(media_type_of(&ftyp("avif")), Some("image/avif"));
    assert_eq!(media_type_of(&ftyp("avis")), Some("image/avif"));
}

#[test]
fn does_not_take_another_riff_or_iso_file_for_an_image() {
    assert_eq!(media_type_of(&riff("WAVEfmt ")), None);
    assert_eq!(media_type_of(&ftyp("mp42")), None);
}

#[test]
fn rejects_truncated_signatures() {
    assert_eq!(media_type_of(&[0x89, b'P']), None);
    assert_eq!(media_type_of(b"RIFF"), None);
    assert_eq!(media_type_of(b""), None);
}

#[test]
fn recognizes_svg_after_a_prolog() {
    assert_eq!(
        media_type_of(b"<svg xmlns=\"http://www.w3.org/2000/svg\"/>"),
        Some("image/svg+xml")
    );
    assert_eq!(media_type_of(b"<svg>"), Some("image/svg+xml"));
    assert_eq!(media_type_of(b"<svg/>"), Some("image/svg+xml"));
    assert_eq!(
        media_type_of(
            "\u{feff}  \n<?xml version=\"1.0\"?>\n<!-- made by hand -->\n<!DOCTYPE svg PUBLIC \"x\" \"y\">\n<SVG/>"
                .as_bytes()
        ),
        Some("image/svg+xml")
    );
}

#[test]
fn does_not_take_other_text_or_markup_for_svg() {
    assert_eq!(media_type_of(b"<html><svg/></html>"), None);
    assert_eq!(media_type_of(b"<svgx/>"), None);
    assert_eq!(media_type_of(b"<svg"), None);
    assert_eq!(media_type_of(b"<?xml never closed"), None);
    assert_eq!(media_type_of(b"hello"), None);
    assert_eq!(media_type_of(&[0xff, 0xfe, b'<']), None);
}

#[test]
fn survives_a_multi_byte_character_cut_at_the_limit() {
    let mut data = b"<svg>".to_vec();
    data.resize(2047, b' ');
    data.extend("é".as_bytes());
    assert_eq!(media_type_of(&data), Some("image/svg+xml"));
}
