// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;

fn spring(mass: f64, stiffness: f64, damping: f64, velocity: f64) -> Spring {
    Spring::new(mass, stiffness, damping, velocity).unwrap_or_default()
}

#[test]
fn starts_at_zero_and_is_exactly_one_once_settled() {
    let s = Spring::default();
    assert_eq!(s.at(0.0), 0.0);
    assert_eq!(s.at(-5.0), 0.0);
    assert_eq!(s.at(f64::NAN), 0.0);
    assert_eq!(s.at(s.duration_ms()), 1.0);
    assert_eq!(s.at(s.duration_ms() * 10.0), 1.0);
}

#[test]
fn the_default_is_the_documented_spring() {
    assert_eq!(Some(Spring::default()), Spring::new(1.0, 100.0, 10.0, 0.0));
}

#[test]
fn settling_times_match_the_reference_values() {
    // Same values as the TypeScript package, which is tested against an independent calculation.
    for (spring, expected) in [
        (spring(1.0, 100.0, 10.0, 0.0), 1271.0),
        (spring(1.0, 100.0, 15.0, 0.0), 800.0),
        (spring(1.0, 100.0, 20.0, 0.0), 924.0),
        (spring(1.0, 100.0, 40.0, 0.0), 2606.0),
        (spring(1.0, 300.0, 15.0, 0.0), 886.0),
        (spring(1.0, 100.0, 20.0, -5.0), 965.0),
    ] {
        assert_eq!(spring.duration_ms(), expected);
    }
}

#[test]
fn overshoots_when_under_damped() {
    let s = spring(1.0, 100.0, 5.0, 0.0);
    let peak = (0..200)
        .map(|i| s.at(f64::from(i) * 5.0))
        .fold(0.0, f64::max);
    assert!(peak > 1.1);
}

#[test]
fn does_not_overshoot_when_critically_or_over_damped() {
    for damping in [20.0, 40.0] {
        let s = spring(1.0, 100.0, damping, 0.0); // 20 is critical for mass 1 and stiffness 100
        let mut t = 0.0;
        while t <= s.duration_ms() {
            assert!(s.at(t) <= 1.0 + 1e-12, "damping {damping} at {t}");
            t += 5.0;
        }
    }
}

#[test]
fn a_damping_ratio_within_a_billionth_of_one_is_critical() {
    let critical = spring(1.0, 100.0, 20.0, 0.0);
    let nearly = spring(1.0, 100.0, 20.0 * (1.0 + 1e-12), 0.0);
    assert_eq!(critical.duration_ms(), nearly.duration_ms());
}

#[test]
fn is_still_moving_just_before_the_duration() {
    for s in [
        spring(1.0, 100.0, 10.0, 0.0),
        spring(1.0, 100.0, 20.0, 0.0),
        spring(1.0, 100.0, 40.0, 0.0),
    ] {
        assert!((1.0 - s.at(s.duration_ms() - 1.0)).abs() > 0.0009);
    }
}

#[test]
fn stiffness_makes_it_faster_and_a_starting_velocity_moves_it_earlier() {
    assert!(
        spring(1.0, 400.0, 20.0, 0.0).duration_ms() < spring(1.0, 100.0, 20.0, 0.0).duration_ms()
    );
    assert!(spring(1.0, 100.0, 10.0, 10.0).at(20.0) > spring(1.0, 100.0, 10.0, 0.0).at(20.0));
}

#[test]
fn a_very_stiff_spring_settles_within_a_millisecond() {
    assert_eq!(spring(1.0, 1e12, 1e7, 0.0).duration_ms(), 1.0);
}

#[test]
fn easing_maps_zero_to_one_onto_the_duration() {
    let s = spring(1.0, 100.0, 15.0, 0.0);
    assert_eq!(s.easing(0.0), 0.0);
    assert_eq!(s.easing(0.25), s.at(s.duration_ms() * 0.25));
    assert_eq!(s.easing(1.0), 1.0);
}

#[test]
fn refuses_non_positive_or_non_finite_parameters() {
    assert_eq!(Spring::new(0.0, 100.0, 10.0, 0.0), None);
    assert_eq!(Spring::new(1.0, -1.0, 10.0, 0.0), None);
    assert_eq!(Spring::new(1.0, 100.0, 0.0, 0.0), None);
    assert_eq!(Spring::new(1.0, 100.0, 10.0, f64::NAN), None);
    assert_eq!(Spring::new(f64::INFINITY, 100.0, 10.0, 0.0), None);
    assert_eq!(Spring::new(f64::NAN, 100.0, 10.0, 0.0), None);
    assert_eq!(Spring::new(1e200, 1e200, 1e200, 0.0), None); // the product overflows
}

#[test]
fn refuses_a_spring_that_never_settles_within_a_finite_time() {
    // Settling would take longer than f64 can count in milliseconds.
    assert_eq!(Spring::new(1.0, 1.0, 1e-306, 0.0), None);
    // A tiny damping is valid as long as the time stays finite (about 4e303 years here).
    assert!(Spring::new(1.0, 1.0, 1e-300, 0.0).is_some_and(|s| s.duration_ms().is_finite()));
}

/// Integrates the spring equation `m·p'' = k·(1 − p) − c·p'` numerically (fourth-order
/// Runge–Kutta, 0.1 ms steps): an independent reference for the exact solutions. Returns the
/// progress at each whole millisecond from `0` to `until_ms`.
fn simulate(mass: f64, stiffness: f64, damping: f64, velocity: f64, until_ms: u32) -> Vec<f64> {
    let dt = 1e-4;
    let accel = |p: f64, v: f64| (stiffness * (1.0 - p) - damping * v) / mass;
    let (mut p, mut v) = (0.0, velocity);
    let mut progress = vec![0.0];
    for _ in 0..until_ms {
        for _ in 0..10 {
            let (k1p, k1v) = (v, accel(p, v));
            let (k2p, k2v) = (
                v + dt / 2.0 * k1v,
                accel(p + dt / 2.0 * k1p, v + dt / 2.0 * k1v),
            );
            let (k3p, k3v) = (
                v + dt / 2.0 * k2v,
                accel(p + dt / 2.0 * k2p, v + dt / 2.0 * k2v),
            );
            let (k4p, k4v) = (v + dt * k3v, accel(p + dt * k3p, v + dt * k3v));
            p += dt / 6.0 * (k1p + 2.0 * k2p + 2.0 * k3p + k4p);
            v += dt / 6.0 * (k1v + 2.0 * k2v + 2.0 * k3v + k4v);
        }
        progress.push(p);
    }
    progress
}

/// (mass, stiffness, damping, initial velocity): under-, critically and over-damped springs,
/// with no, positive and negative starting speed.
const SPRINGS: [(f64, f64, f64, f64); 9] = [
    (1.0, 100.0, 10.0, 0.0),
    (1.0, 100.0, 10.0, 6.0),
    (1.0, 100.0, 20.0, 0.0),
    (1.0, 100.0, 20.0, -5.0),
    (1.0, 100.0, 40.0, 0.0),
    (1.0, 100.0, 40.0, 8.0),
    (1.0, 300.0, 15.0, 3.0),
    (2.0, 50.0, 3.0, 4.0),
    (0.5, 200.0, 30.0, -2.0),
];

#[test]
fn follows_the_numerical_solution_of_the_spring_equation() {
    for (mass, stiffness, damping, velocity) in SPRINGS {
        let spring = spring(mass, stiffness, damping, velocity);
        let reference = simulate(mass, stiffness, damping, velocity, 3000);
        for (ms, expected) in reference.iter().enumerate().step_by(7) {
            let at = f64::from(u32::try_from(ms).unwrap_or(0));
            if at < spring.duration_ms() {
                let got = spring.at(at);
                assert!(
                    (got - expected).abs() < 1e-5,
                    "{mass} {stiffness} {damping} {velocity} at {ms} ms: {got} vs {expected}"
                );
            }
        }
    }
}

#[test]
fn the_duration_is_the_last_moment_the_spring_is_further_than_a_thousandth_from_its_target() {
    for (mass, stiffness, damping, velocity) in SPRINGS {
        let spring = spring(mass, stiffness, damping, velocity);
        let reference = simulate(mass, stiffness, damping, velocity, 15_000);
        let last_unsettled = reference
            .iter()
            .rposition(|p| (1.0 - p).abs() >= 0.001)
            .unwrap_or(0);
        let expected = f64::from(u32::try_from(last_unsettled).unwrap_or(0)) + 1.0;
        assert!(
            (spring.duration_ms() - expected).abs() <= 1.0,
            "{mass} {stiffness} {damping} {velocity}: {} vs {expected}",
            spring.duration_ms()
        );
    }
}
