// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-4
}

#[test]
fn linear_is_the_identity_clamped() {
    let linear = Easing::linear();
    assert_eq!(linear.at(0.3), 0.3);
    assert_eq!(linear.at(-1.0), 0.0);
    assert_eq!(linear.at(2.0), 1.0);
    assert_eq!(linear.at(f64::NAN), 0.0);
}

#[test]
fn cubic_bezier_matches_the_css_curves() {
    let ease = Easing::cubic_bezier(0.25, 0.1, 0.25, 1.0).unwrap_or_else(Easing::linear);
    assert!(close(ease.at(0.5), 0.8024));
    assert!(close(ease.at(0.25), 0.4085));
    let straight = Easing::cubic_bezier(0.0, 0.0, 1.0, 1.0).unwrap_or_else(Easing::linear);
    for t in [0.1, 0.33, 0.9] {
        assert!((straight.at(t) - t).abs() < 1e-9);
    }
}

#[test]
fn cubic_bezier_pins_the_ends_and_can_overshoot() {
    let back = Easing::cubic_bezier(0.34, 1.56, 0.64, 1.0).unwrap_or_else(Easing::linear);
    assert_eq!(back.at(0.0), 0.0);
    assert_eq!(back.at(1.0), 1.0);
    assert!(back.at(0.5) > 1.0);
}

#[test]
fn cubic_bezier_refuses_an_x_outside_the_unit_range_or_a_non_finite_value() {
    assert_eq!(Easing::cubic_bezier(-0.1, 0.0, 1.0, 1.0), None);
    assert_eq!(Easing::cubic_bezier(0.0, 0.0, 1.1, 1.0), None);
    assert_eq!(Easing::cubic_bezier(f64::NAN, 0.0, 1.0, 1.0), None);
    assert_eq!(Easing::cubic_bezier(0.0, f64::INFINITY, 1.0, 1.0), None);
    assert_eq!(Easing::cubic_bezier(0.0, 0.0, 1.0, f64::NAN), None);
}

#[test]
fn steps_jump_at_the_end_by_default() {
    let four = Easing::steps(4, StepPosition::JumpEnd).unwrap_or_else(Easing::linear);
    let values: Vec<f64> = [0.0, 0.24, 0.25, 0.3, 0.99, 1.0]
        .iter()
        .map(|&t| four.at(t))
        .collect();
    assert_eq!(values, [0.0, 0.0, 0.25, 0.25, 0.75, 1.0]);
}

#[test]
fn steps_can_jump_at_the_start_at_both_ends_or_at_neither() {
    let at = |count, position, t| Easing::steps(count, position).map_or(f64::NAN, |e| e.at(t));
    assert_eq!(
        [0.0, 0.3, 1.0].map(|t| at(4, StepPosition::JumpStart, t)),
        [0.25, 0.5, 1.0]
    );
    assert_eq!(
        [0.0, 0.5, 0.99, 1.0].map(|t| at(2, StepPosition::JumpBoth, t)),
        [1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0, 1.0]
    );
    assert_eq!(
        [0.0, 0.49, 0.5, 1.0].map(|t| at(2, StepPosition::JumpNone, t)),
        [0.0, 0.0, 1.0, 1.0]
    );
}

#[test]
fn steps_refuse_a_count_that_is_too_small() {
    assert_eq!(Easing::steps(0, StepPosition::JumpEnd), None);
    assert_eq!(Easing::steps(1, StepPosition::JumpNone), None);
    assert!(Easing::steps(1, StepPosition::JumpEnd).is_some());
    assert!(Easing::steps(2, StepPosition::JumpNone).is_some());
}

#[test]
fn parse_and_from_str_read_css() {
    assert_eq!(Easing::parse("linear"), Ok(Easing::linear()));
    assert_eq!("ease-out".parse::<Easing>(), Easing::parse("ease-out"));
    assert_eq!(
        "bounce".parse::<Easing>(),
        Err(ParseEasingError::UnknownFunction)
    );
}

#[test]
fn stops_interpolate_between_their_points_and_hold_the_edges() {
    let easing = Easing::from_stops(vec![
        Stop {
            input: 0.2,
            output: 0.2,
        },
        Stop {
            input: 0.8,
            output: 0.8,
        },
    ]);
    assert_eq!(easing.at(0.1), 0.2);
    assert_eq!(easing.at(0.9), 0.8);
    assert!((easing.at(0.5) - 0.5).abs() < 1e-12);
    assert_eq!(easing.at(0.2), 0.2);
    assert_eq!(easing.at(0.8), 0.8);
}

#[test]
fn stops_with_equal_inputs_make_a_jump() {
    let easing = Easing::from_stops(vec![
        Stop {
            input: 0.0,
            output: 0.0,
        },
        Stop {
            input: 0.6,
            output: 0.0,
        },
        Stop {
            input: 0.6,
            output: 1.0,
        },
        Stop {
            input: 1.0,
            output: 1.0,
        },
    ]);
    assert_eq!(easing.at(0.59), 0.0);
    assert_eq!(easing.at(0.6), 1.0);
}
