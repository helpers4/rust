// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

#[test]
fn builds_an_aligned_table() {
    let out = table(
        &["Name", "Stars"],
        &[vec!["typescript", "1"], vec!["rust", "0"]],
    );
    assert_eq!(
        out,
        "| Name       | Stars |\n| ---------- | ----- |\n| typescript | 1     |\n| rust       | 0     |\n"
    );
}

#[test]
fn the_separator_is_at_least_three_dashes() {
    assert_eq!(table(&["a"], &[vec!["b"]]), "| a   |\n| --- |\n| b   |\n");
}

#[test]
fn pads_short_rows_and_cuts_long_ones() {
    let out = table(&["a", "b"], &[vec!["1"], vec!["1", "2", "3"]]);
    assert_eq!(
        out,
        "| a   | b   |\n| --- | --- |\n| 1   |     |\n| 1   | 2   |\n"
    );
}

#[test]
fn a_table_without_rows_is_just_the_header() {
    assert_eq!(
        table(&["a", "b"], &Vec::<Vec<&str>>::new()),
        "| a   | b   |\n| --- | --- |\n"
    );
}

#[test]
fn no_headers_gives_an_empty_string() {
    assert_eq!(table(&Vec::<&str>::new(), &[vec!["x"]]), "");
}

#[test]
fn keeps_cells_from_breaking_the_table() {
    let out = table(&["a|b"], &[vec!["x|y\nz"]]);
    assert_eq!(out, "| a\\|b   |\n| ------ |\n| x\\|y z |\n");
}

#[test]
fn measures_width_in_characters() {
    let out = table(&["é"], &[vec!["日本語日本語"]]);
    assert_eq!(out, "| é      |\n| ------ |\n| 日本語日本語 |\n");
}

#[test]
fn accepts_owned_strings() {
    let headers = vec!["h".to_string()];
    let rows = vec![vec!["c".to_string()]];
    assert_eq!(table(&headers, &rows), "| h   |\n| --- |\n| c   |\n");
}
