// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

#![allow(clippy::float_cmp)]

use super::super::_test_support::*;
use super::*;

fn read(tables: &[Table]) -> FontInfo {
    read_font(&sfnt(tables)).unwrap()
}

#[test]
fn reads_family_style_postscript_name_weight_and_italic() {
    let info = read(&typical_tables(
        "Neutrif Pro",
        "Semi Bold Italic",
        600,
        0x0001,
    ));
    assert_eq!(info.family, "Neutrif Pro");
    assert_eq!(info.style, "Semi Bold Italic");
    assert_eq!(
        info.postscript_name.as_deref(),
        Some("NeutrifPro-SemiBoldItalic")
    );
    assert_eq!(info.weight, 600.0);
    assert_eq!(info.weight_range, None);
    assert!(info.italic);
}

#[test]
fn a_regular_font_is_not_italic() {
    let info = read(&typical_tables("Inter", "Regular", 400, 0x0040));
    assert!(!info.italic);
    assert_eq!(info.weight, 400.0);
}

#[test]
fn the_typographic_names_come_first() {
    let name = name_table(&[
        windows(1, "Inter Semi Bold"),
        windows(2, "Regular"),
        windows(16, "Inter"),
        windows(17, "Semi Bold"),
    ]);
    let info = read(&[(*b"name", name)]);
    assert_eq!(
        (info.family.as_str(), info.style.as_str()),
        ("Inter", "Semi Bold")
    );
}

#[test]
fn a_font_without_a_style_name_has_an_empty_style() {
    let info = read(&[(*b"name", name_table(&[windows(1, "Inter")]))]);
    assert_eq!(info.style, "");
    assert_eq!(info.postscript_name, None);
    assert_eq!(info.weight, 400.0);
    assert!(!info.italic);
}

#[test]
fn the_weight_comes_from_the_style_name_without_an_os2_table() {
    let name = name_table(&[windows(1, "Inter"), windows(2, "Extra Bold Oblique")]);
    let info = read(&[(*b"name", name)]);
    assert_eq!(info.weight, 800.0);
    assert!(info.italic);
}

#[test]
fn the_head_flags_say_bold_and_italic() {
    let name = name_table(&[windows(1, "Inter"), windows(2, "Whatever")]);
    let bold = read(&[(*b"name", name.clone()), (*b"head", head_table(1))]);
    assert_eq!(bold.weight, 700.0);
    assert!(!bold.italic);
    let italic = read(&[(*b"name", name), (*b"head", head_table(2))]);
    assert!(italic.italic);
    assert_eq!(italic.weight, 400.0);
}

#[test]
fn an_os2_weight_out_of_range_or_a_short_table_is_ignored() {
    let name = name_table(&[windows(1, "Inter"), windows(2, "Light")]);
    let zero = read(&[(*b"name", name.clone()), (*b"OS/2", os2_table(0, 0))]);
    assert_eq!(zero.weight, 300.0);
    let huge = read(&[(*b"name", name.clone()), (*b"OS/2", os2_table(1001, 0))]);
    assert_eq!(huge.weight, 300.0);
    let short = read(&[(*b"name", name.clone()), (*b"OS/2", vec![0; 3])]);
    assert_eq!(short.weight, 300.0);
    let no_flags = read(&[
        (*b"name", name),
        (*b"OS/2", os2_table(500, 0)[..40].to_vec()),
    ]);
    assert_eq!(no_flags.weight, 500.0);
    assert!(!no_flags.italic);
}

#[test]
fn the_oblique_flag_counts_as_italic() {
    let info = read(&typical_tables("Inter", "Regular", 400, 0x0200));
    assert!(info.italic);
}

#[test]
fn a_short_head_table_has_no_flags() {
    let name = name_table(&[windows(1, "Inter"), windows(2, "Regular")]);
    let info = read(&[(*b"name", name), (*b"head", vec![0; 10])]);
    assert_eq!(info.weight, 400.0);
    assert!(!info.italic);
}

#[test]
fn reads_the_weight_axis_of_a_variable_font() {
    let mut tables = typical_tables("Inter", "Regular", 400, 0);
    tables.push((
        *b"fvar",
        fvar(&[
            (b"opsz", 75.0, 100.0, 125.0),
            (b"wght", 100.0, 400.0, 900.0),
        ]),
    ));
    let info = read(&tables);
    assert_eq!(info.weight_range, Some((100.0, 900.0)));
    assert_eq!(info.to_face().weight_max, 900.0);
}

#[test]
fn a_variable_font_without_a_weight_axis_has_no_range() {
    let mut tables = typical_tables("Inter", "Regular", 400, 0);
    tables.push((*b"fvar", fvar(&[(b"opsz", 75.0, 100.0, 125.0)])));
    assert_eq!(read(&tables).weight_range, None);
    let mut truncated = typical_tables("Inter", "Regular", 400, 0);
    truncated.push((
        *b"fvar",
        fvar(&[(b"wght", 100.0, 400.0, 900.0)])[..30].to_vec(),
    ));
    assert_eq!(read(&truncated).weight_range, None);
    let mut short = typical_tables("Inter", "Regular", 400, 0);
    short.push((*b"fvar", vec![0; 4]));
    assert_eq!(read(&short).weight_range, None);
}

#[test]
fn a_windows_english_record_beats_the_others() {
    let name = name_table(&[
        (1, 0, 0, 1, b"Mac Name".to_vec()),
        (3, 1, 0x040c, 1, utf16("Nom Français")),
        (0, 3, 0, 1, utf16("Unicode Name")),
        windows(1, "English Name"),
        (3, 1, 0x0409, 6, utf16("PS")),
    ]);
    let info = read(&[(*b"name", name)]);
    assert_eq!(info.family, "English Name");
    let without_english = name_table(&[
        (1, 0, 0, 1, b"Mac Name".to_vec()),
        (0, 3, 0, 1, utf16("Unicode Name")),
    ]);
    assert_eq!(read(&[(*b"name", without_english)]).family, "Unicode Name");
}

#[test]
fn a_macintosh_record_is_decoded_as_ascii() {
    let name = name_table(&[(1, 0, 0, 1, b"Bar\x8e".to_vec())]);
    assert_eq!(read(&[(*b"name", name)]).family, "Bar\u{fffd}");
}

#[test]
fn an_unpaired_surrogate_becomes_a_replacement_character() {
    let name = name_table(&[(3, 1, 0x0409, 1, vec![0xd8, 0x00, 0x00, 0x41])]);
    assert_eq!(read(&[(*b"name", name)]).family, "\u{fffd}A");
}

#[test]
fn skips_records_of_other_platforms_and_out_of_range_strings() {
    let mut name = name_table(&[(2, 1, 0, 1, b"Ignored".to_vec()), windows(1, "Kept")]);
    // A record whose string lies past the end of the table.
    let broken = name_table(&[windows(1, "Kept"), windows(1, "Lost")]);
    assert_eq!(read(&[(*b"name", name.clone())]).family, "Kept");
    let mut cut = broken;
    cut.truncate(cut.len() - 8);
    assert_eq!(read(&[(*b"name", cut)]).family, "Kept");
    // Records announced but missing from the table.
    name.truncate(6 + 12);
    assert_eq!(
        read_font(&sfnt(&[(*b"name", name)])),
        Err(FontError::MissingFamily)
    );
}

#[test]
fn a_font_without_a_name_table_or_a_family_is_an_error() {
    assert_eq!(
        read_font(&sfnt(&[(*b"head", head_table(0))])),
        Err(FontError::MissingNameTable)
    );
    assert_eq!(
        read_font(&sfnt(&[(*b"name", name_table(&[windows(2, "Regular")]))])),
        Err(FontError::MissingFamily)
    );
    assert_eq!(
        read_font(&sfnt(&[(*b"name", vec![0; 3])])),
        Err(FontError::MissingFamily)
    );
}

#[test]
fn reads_opentype_true_type_and_collection_files() {
    let tables = typical_tables("Inter", "Bold", 700, 0);
    for version in [*b"OTTO", *b"true"] {
        let bytes = sfnt_at(&tables, 0, version);
        assert_eq!(read_font(&bytes).unwrap().family, "Inter");
    }
    assert_eq!(read_font(&collection(&tables)).unwrap().weight, 700.0);
}

#[test]
fn reads_woff_files() {
    let info = read_font(&woff(&typical_tables("Inter", "Bold", 700, 0))).unwrap();
    assert_eq!((info.family.as_str(), info.weight), ("Inter", 700.0));
}

#[test]
fn rejects_other_formats_and_woff2() {
    assert_eq!(read_font(b""), Err(FontError::Truncated));
    assert_eq!(read_font(b"abc"), Err(FontError::Truncated));
    assert_eq!(read_font(b"%PDF-1.4"), Err(FontError::UnknownFormat));
    assert_eq!(read_font(b"wOF2abcdefgh"), Err(FontError::UnsupportedWoff2));
}

#[test]
fn rejects_truncated_files() {
    let bytes = sfnt(&typical_tables("Inter", "Bold", 700, 0));
    // The last table is followed by up to three bytes of padding.
    for len in 0..bytes.len() - 3 {
        assert!(read_font(&bytes[..len]).is_err(), "{len}");
    }
    let collection = collection(&typical_tables("Inter", "Bold", 700, 0));
    for len in 0..collection.len() - 3 {
        assert!(read_font(&collection[..len]).is_err(), "{len}");
    }
    assert_eq!(
        read_font(b"ttcf\0\x01\0\0\0\0\0\x01"),
        Err(FontError::Truncated)
    );
}
