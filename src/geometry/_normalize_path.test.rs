// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use crate::geometry::parse_path;

fn normalized(d: &str) -> String {
    crate::geometry::format_path(&normalize_path(&parse_path(d).unwrap()), 6)
}

#[test]
fn reads_relative_commands_from_the_current_point() {
    assert_eq!(
        normalized("m1 1 l2 0 c1 0 1 1 0 2 q1 1 2 0"),
        "M1 1 L3 1 C4 1 4 2 3 3 Q4 4 5 3"
    );
}

#[test]
fn expands_h_and_v() {
    assert_eq!(normalized("M1 1 H5 V7 h1 v-1"), "M1 1 L5 1 L5 7 L6 7 L6 6");
}

#[test]
fn reflects_the_previous_control_point_for_s_and_t() {
    assert_eq!(
        normalized("M0 0 C0 1 1 2 2 2 S4 3 4 2"),
        "M0 0 C0 1 1 2 2 2 C3 2 4 3 4 2"
    );
    assert_eq!(normalized("M0 0 Q1 1 2 0 T4 0"), "M0 0 Q1 1 2 0 Q3 -1 4 0");
}

#[test]
fn starts_s_and_t_from_the_current_point_when_no_curve_precedes() {
    assert_eq!(normalized("M1 1 S2 2 3 3"), "M1 1 C1 1 2 2 3 3");
    assert_eq!(normalized("M1 1 T3 3"), "M1 1 Q1 1 3 3");
    assert_eq!(
        normalized("M0 0 C0 1 1 2 2 2 L3 3 S4 4 5 5"),
        "M0 0 C0 1 1 2 2 2 L3 3 C3 3 4 4 5 5"
    );
    assert_eq!(
        normalized("M0 0 Q1 1 2 2 L3 3 T4 4"),
        "M0 0 Q1 1 2 2 L3 3 Q3 3 4 4"
    );
}

#[test]
fn goes_back_to_the_start_of_the_subpath_after_a_close() {
    assert_eq!(normalized("M1 1 L3 1 z l1 1"), "M1 1 L3 1 Z L2 2");
}

#[test]
fn turns_a_zero_radius_arc_into_a_line_and_skips_an_arc_to_the_same_point() {
    assert_eq!(normalized("M0 0 A0 5 0 0 1 5 5"), "M0 0 L5 5");
    assert_eq!(normalized("M0 0 A5 0 0 0 1 5 5"), "M0 0 L5 5");
    assert_eq!(normalized("M1 1 A5 5 0 0 1 1 1"), "M1 1");
}

#[test]
fn turns_an_arc_into_cubics_that_reach_its_end_point() {
    let commands = normalize_path(&parse_path("M0 0 A5 5 0 0 1 10 0").unwrap());
    assert_eq!(commands.len(), 3); // a move and two quarter turns
    let last = commands.last().unwrap();
    assert_eq!(
        (last.letter, last.values[4], last.values[5]),
        ('C', 10.0, 0.0)
    );
    // The middle of a half circle on the circle, above the chord.
    let mid = &commands[1].values;
    assert!((((mid[4] - 5.0).hypot(mid[5])) - 5.0).abs() < 1e-9);
    assert!(mid[5] < 0.0);
}

#[test]
fn follows_the_sweep_and_large_arc_flags() {
    let count = |flags: &str| {
        normalize_path(&parse_path(&format!("M0 0 A5 5 0 {flags} 6 0")).unwrap()).len()
    };
    assert!(count("0 0") < count("1 1"));
    let top = |flags: &str| {
        normalize_path(&parse_path(&format!("M0 0 A5 5 0 {flags} 6 0")).unwrap())
            .iter()
            .flat_map(|c| {
                c.values
                    .iter()
                    .skip(1)
                    .step_by(2)
                    .copied()
                    .collect::<Vec<_>>()
            })
            .fold(f64::INFINITY, f64::min)
    };
    assert!(top("0 1") < top("0 0"));
}

#[test]
fn scales_up_radii_that_are_too_small() {
    let commands = normalize_path(&parse_path("M0 0 A1 1 0 0 1 10 0").unwrap());
    let last = commands.last().unwrap();
    assert_eq!((last.values[4], last.values[5]), (10.0, 0.0));
    let mid = &commands[1].values;
    assert!((((mid[4] - 5.0).hypot(mid[5])) - 5.0).abs() < 1e-9);
}

#[test]
fn handles_a_rotated_ellipse_and_every_direction_of_the_chord() {
    let ends_at = |d: &str, end: (f64, f64)| {
        let commands = normalize_path(&parse_path(d).unwrap());
        let last = commands.last().unwrap();
        assert!((last.values[4] - end.0).abs() < 1e-9 && (last.values[5] - end.1).abs() < 1e-9);
    };
    ends_at("M0 0 A10 5 30 0 1 8 4", (8.0, 4.0));
    for (large, sweep) in [("0", "0"), ("0", "1"), ("1", "0"), ("1", "1")] {
        for degrees in (10..360).step_by(35) {
            let radians = f64::from(degrees).to_radians();
            let end = (5.0 * radians.cos(), 5.0 * radians.sin());
            ends_at(
                &format!("M5 0 A5 5 0 {large} {sweep} {} {}", end.0, end.1),
                end,
            );
        }
    }
}
