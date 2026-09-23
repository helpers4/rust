// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

const ALL: [Weekday; 7] = [
    Weekday::Monday,
    Weekday::Tuesday,
    Weekday::Wednesday,
    Weekday::Thursday,
    Weekday::Friday,
    Weekday::Saturday,
    Weekday::Sunday,
];

#[test]
fn names_are_english_and_capitalized() {
    let names: Vec<&str> = ALL.iter().map(|d| d.name()).collect();
    assert_eq!(
        names,
        [
            "Monday",
            "Tuesday",
            "Wednesday",
            "Thursday",
            "Friday",
            "Saturday",
            "Sunday"
        ]
    );
}

#[test]
fn iso_numbers_run_from_one_to_seven() {
    let numbers: Vec<u8> = ALL.iter().map(|d| d.iso_number()).collect();
    assert_eq!(numbers, [1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn only_saturday_and_sunday_are_the_weekend() {
    let weekend: Vec<bool> = ALL.iter().map(|d| d.is_weekend()).collect();
    assert_eq!(weekend, [false, false, false, false, false, true, true]);
}

#[test]
fn the_index_wraps_around_the_week() {
    for (i, day) in ALL.iter().enumerate() {
        let i = i64::try_from(i).unwrap();
        assert_eq!(Weekday::from_monday_index(i), *day);
        assert_eq!(Weekday::from_monday_index(i + 7), *day);
        assert_eq!(Weekday::from_monday_index(i - 7), *day);
    }
}
