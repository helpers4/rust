// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use crate::geometry::parse_path;

fn cmd(letter: char, values: &[f64]) -> PathCommand {
    PathCommand::new(letter, values.to_vec())
}

#[test]
fn writes_letters_glued_to_their_first_number_and_a_space_elsewhere() {
    let d = format_path(
        &[
            cmd('M', &[0.0, 0.5]),
            cmd('L', &[10.0, -2.0]),
            cmd('Z', &[]),
        ],
        6,
    );
    assert_eq!(d, "M0 0.5 L10 -2 Z");
}

#[test]
fn rounds_to_the_precision_and_drops_trailing_zeros_and_negative_zero() {
    assert_eq!(
        format_path(&[cmd('L', &[1.0 / 3.0, -0.000_000_1, 2.5])], 6),
        "L0.333333 0 2.5"
    );
    assert_eq!(
        format_path(&[cmd('L', &[1.0 / 3.0, 1.23456])], 2),
        "L0.33 1.23"
    );
}

#[test]
fn keeps_the_zeros_of_a_whole_number_without_decimals() {
    assert_eq!(format_path(&[cmd('L', &[100.0, 20.0])], 0), "L100 20");
}

#[test]
fn writes_nothing_for_no_command() {
    assert_eq!(format_path(&[], 6), "");
}

#[test]
fn round_trips_parsed_data() {
    let d = "M0 0 L10 0 A5 5 0 0 1 10 10 Z";
    assert_eq!(format_path(&parse_path(d).unwrap(), 6), d);
}
