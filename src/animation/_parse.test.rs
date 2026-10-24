// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn sample(css: &str) -> Vec<f64> {
    let easing = parse(css).unwrap_or_else(|_| Easing::linear());
    [0.0, 0.1, 0.25, 0.5, 0.75, 0.9, 1.0]
        .iter()
        .map(|&t| easing.at(t))
        .collect()
}

fn same(css: &str, reference: &Easing) {
    let expected: Vec<f64> = [0.0, 0.1, 0.25, 0.5, 0.75, 0.9, 1.0]
        .iter()
        .map(|&t| reference.at(t))
        .collect();
    assert_eq!(sample(css), expected, "{css}");
}

fn bezier(x1: f64, y1: f64, x2: f64, y2: f64) -> Easing {
    Easing::cubic_bezier(x1, y1, x2, y2).unwrap_or_else(Easing::linear)
}

fn steps_of(count: u32, position: StepPosition) -> Easing {
    Easing::steps(count, position).unwrap_or_else(Easing::linear)
}

#[test]
fn reads_every_keyword() {
    same("linear", &Easing::linear());
    same("ease", &bezier(0.25, 0.1, 0.25, 1.0));
    same("ease-in", &bezier(0.42, 0.0, 1.0, 1.0));
    same("ease-out", &bezier(0.0, 0.0, 0.58, 1.0));
    same("ease-in-out", &bezier(0.42, 0.0, 0.58, 1.0));
    same("step-start", &steps_of(1, StepPosition::JumpStart));
    same("step-end", &steps_of(1, StepPosition::JumpEnd));
}

#[test]
fn ignores_case_and_spaces() {
    assert!(parse("  Ease-Out ").is_ok());
    same(
        "CUBIC-BEZIER( 0.1 , 0.7,1.0, .1 )",
        &bezier(0.1, 0.7, 1.0, 0.1),
    );
}

#[test]
fn reads_cubic_bezier_with_overshoot_signs_and_exponents() {
    assert!(parse("cubic-bezier(0.34, 1.56, 0.64, 1)").is_ok_and(|e| e.at(0.5) > 1.0));
    same("cubic-bezier(0, 0, 1e0, 1)", &bezier(0.0, 0.0, 1.0, 1.0));
    same(
        "cubic-bezier(+0.5, -1, 0.5, 2.)",
        &bezier(0.5, -1.0, 0.5, 2.0),
    );
}

#[test]
fn rejects_invalid_cubic_bezier() {
    for css in [
        "cubic-bezier(1.1, 0, 1, 1)",
        "cubic-bezier(0, 0, -0.1, 1)",
        "cubic-bezier(0, 0, 1)",
        "cubic-bezier(0, 0, 1, 1, 1)",
        "cubic-bezier(0, a, 1, 1)",
        "cubic-bezier(0, inf, 1, 1)",
        "cubic-bezier(0, nan, 1, 1)",
        "cubic-bezier(0, 1e999, 1, 1)",
        "cubic-bezier()",
    ] {
        assert_eq!(parse(css), Err(ParseEasingError::InvalidArguments), "{css}");
    }
}

#[test]
fn reads_steps_with_every_position() {
    same("steps(4)", &steps_of(4, StepPosition::JumpEnd));
    same(
        "steps(4, jump-start)",
        &steps_of(4, StepPosition::JumpStart),
    );
    same("steps(4, start)", &steps_of(4, StepPosition::JumpStart));
    same("steps(4, jump-end)", &steps_of(4, StepPosition::JumpEnd));
    same("steps(4, end)", &steps_of(4, StepPosition::JumpEnd));
    same("steps(3, jump-none)", &steps_of(3, StepPosition::JumpNone));
    same("steps(3, jump-both)", &steps_of(3, StepPosition::JumpBoth));
}

#[test]
fn steps_accepts_an_explicit_plus_sign_like_a_css_integer() {
    same("steps(+4)", &steps_of(4, StepPosition::JumpEnd));
    same("steps(+3, jump-both)", &steps_of(3, StepPosition::JumpBoth));
}

#[test]
fn rejects_invalid_steps() {
    for css in [
        "steps(0)",
        "steps(1, jump-none)",
        "steps(2.5)",
        "steps(-1)",
        "steps(2, middle)",
        "steps(2, end, 3)",
        "steps()",
        "steps(99999999999)",
        "steps(+)",
        "steps(++4)",
        "steps(4.0)",
        "steps(1e2)",
        "steps(0x4)",
        "steps( )",
    ] {
        assert_eq!(parse(css), Err(ParseEasingError::InvalidArguments), "{css}");
    }
}

fn at(css: &str, t: f64) -> f64 {
    parse(css).map_or(f64::NAN, |e| e.at(t))
}

#[test]
fn reads_linear_with_explicit_missing_and_double_inputs() {
    assert!((at("linear(0, 0.9 50%, 1)", 0.25) - 0.45).abs() < 1e-12);
    assert!((at("linear(0, 0.25, 1)", 0.75) - 0.625).abs() < 1e-12);
    assert!((at("linear(0, 0.2, 0.6, 1 80%)", 0.4) - 0.4).abs() < 1e-12);
    assert_eq!(at("linear(0 0% 50%, 1)", 0.25), 0.0);
    assert!((at("linear(0 0% 50%, 1)", 0.75) - 0.5).abs() < 1e-12);
}

#[test]
fn linear_inputs_never_go_backwards_and_the_edges_are_held() {
    assert_eq!(at("linear(0, 0 60%, 1 40%, 1)", 0.59), 0.0);
    assert_eq!(at("linear(0, 0 60%, 1 40%, 1)", 0.6), 1.0);
    assert_eq!(at("linear(0.2 20%, 0.8 80%)", 0.1), 0.2);
    assert_eq!(at("linear(0.2 20%, 0.8 80%)", 0.9), 0.8);
    assert!((at("linear(0.2 20%, 0.8 80%)", 0.5) - 0.5).abs() < 1e-12);
}

#[test]
fn linear_spreads_several_missing_inputs_evenly_between_their_neighbors() {
    // Stops at 0, ?, ?, 90%, with outputs 0, 1, 2, 3: the two unknown ones sit at 30% and 60%.
    assert!((at("linear(0, 1, 2, 3 90%)", 0.3) - 1.0).abs() < 1e-12);
    assert!((at("linear(0, 1, 2, 3 90%)", 0.6) - 2.0).abs() < 1e-12);
    // The last stop defaults to 100%.
    assert!((at("linear(0, 1 50%, 2)", 1.0) - 2.0).abs() < 1e-12);
}

#[test]
fn linear_spreads_missing_inputs_from_the_previous_stop_not_from_zero() {
    // 0 at 0%, 1 at 20%, two stops without an input, 4 at 80%: they sit at 40% and 60%.
    let css = "linear(0 0%, 1 20%, 2, 3, 4 80%)";
    assert!((at(css, 0.2) - 1.0).abs() < 1e-12);
    assert!((at(css, 0.4) - 2.0).abs() < 1e-12);
    assert!((at(css, 0.6) - 3.0).abs() < 1e-12);
    assert!((at(css, 0.8) - 4.0).abs() < 1e-12);
}

#[test]
fn rejects_invalid_linear() {
    for css in [
        "linear(1)",
        "linear()",
        "linear(0, x)",
        "linear(0, 1 50)",
        "linear(0, 1 a%)",
        "linear(0 10% 20% 30%, 1)",
        "linear(0, 1 %)",
    ] {
        assert_eq!(parse(css), Err(ParseEasingError::InvalidArguments), "{css}");
    }
}

#[test]
fn rejects_anything_else_as_an_unknown_function() {
    for css in [
        "bounce",
        "",
        "spring(1, 100, 10, 0)",
        "ease-out()",
        "toString",
        "cubic-bezier(0,0,1,1",
        "(0, 1)",
        "Linear (0, 1)",
        "cubic-bezier(1(0), 0, 1, 1)",
        "cubic bezier(0, 0, 1, 1)",
    ] {
        assert_eq!(parse(css), Err(ParseEasingError::UnknownFunction), "{css}");
    }
}
