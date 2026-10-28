// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn cmd(letter: char, values: &[f64]) -> PathCommand {
    PathCommand::new(letter, values.to_vec())
}

#[test]
fn blank_data_has_no_command() {
    assert_eq!(parse_path(""), Ok(vec![]));
    assert_eq!(parse_path("  ,\n"), Ok(vec![]));
}

#[test]
fn keeps_the_letters_and_their_case() {
    assert_eq!(
        parse_path("M0 0h10v10z"),
        Ok(vec![
            cmd('M', &[0.0, 0.0]),
            cmd('h', &[10.0]),
            cmd('v', &[10.0]),
            cmd('z', &[]),
        ])
    );
}

#[test]
fn splits_a_repeated_command_a_repeated_move_being_a_line() {
    assert_eq!(
        parse_path("M0 0 1 1 2 2L3 3 4 4"),
        Ok(vec![
            cmd('M', &[0.0, 0.0]),
            cmd('L', &[1.0, 1.0]),
            cmd('L', &[2.0, 2.0]),
            cmd('L', &[3.0, 3.0]),
            cmd('L', &[4.0, 4.0]),
        ])
    );
    let letters: Vec<char> = parse_path("m0 0 1 1 2 2")
        .unwrap()
        .iter()
        .map(|c| c.letter)
        .collect();
    assert_eq!(letters, ['m', 'l', 'l']);
}

#[test]
fn reads_glued_numbers_exponents_and_any_separator() {
    let values: Vec<Vec<f64>> = parse_path("M1-2.5.5.5L1e-1,+2E2")
        .unwrap()
        .into_iter()
        .map(|c| c.values)
        .collect();
    assert_eq!(values, [vec![1.0, -2.5], vec![0.5, 0.5], vec![0.1, 200.0]]);
}

#[test]
fn reads_arc_flags_glued_to_the_next_number() {
    assert_eq!(
        parse_path("M0 0a1 1 0 011 1"),
        Ok(vec![
            cmd('M', &[0.0, 0.0]),
            cmd('a', &[1.0, 1.0, 0.0, 0.0, 1.0, 1.0, 1.0])
        ])
    );
}

#[test]
fn accepts_numbers_like_3_dot_and_dot_5() {
    assert_eq!(parse_path("M3. .5"), Ok(vec![cmd('M', &[3.0, 0.5])]));
}

#[test]
fn an_exponent_without_digits_is_not_part_of_the_number() {
    assert_eq!(
        parse_path("M1e 2"),
        Err(ParsePathError::ExpectedNumber { index: 2 })
    );
    assert_eq!(
        parse_path("M1e+ 2"),
        Err(ParsePathError::ExpectedNumber { index: 2 })
    );
}

#[test]
fn rejects_data_that_does_not_start_with_a_move() {
    assert_eq!(parse_path("L0 0"), Err(ParsePathError::MissingMove));
}

#[test]
fn rejects_an_unknown_letter() {
    assert_eq!(
        parse_path("M0 0 X"),
        Err(ParsePathError::UnknownCommand { index: 5 })
    );
}

#[test]
fn rejects_missing_numbers_and_bad_flags() {
    assert_eq!(
        parse_path("M0"),
        Err(ParsePathError::ExpectedNumber { index: 2 })
    );
    assert_eq!(
        parse_path("M0 -"),
        Err(ParsePathError::ExpectedNumber { index: 3 })
    );
    assert_eq!(
        parse_path("M0 ."),
        Err(ParsePathError::ExpectedNumber { index: 3 })
    );
    assert_eq!(
        parse_path("M0 0 A1 1 0 2 1 1 1"),
        Err(ParsePathError::ExpectedFlag { index: 12 })
    );
    assert_eq!(
        parse_path("M0 0 A1 1 0"),
        Err(ParsePathError::ExpectedFlag { index: 11 })
    );
}
