// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

proptest! {
    /// The envelope is what the settling search relies on: at any time it must be at least the
    /// absolute displacement, in every damping regime (a factor of the critical damping: under,
    /// exactly critical, over).
    #[test]
    fn the_envelope_bounds_the_displacement_in_every_regime(
        mass in 0.1f64..10.0,
        stiffness in 10.0f64..1000.0,
        regime in prop::sample::select(vec![0.2f64, 0.5, 0.9, 1.0, 1.1, 2.0, 5.0]),
        velocity in -30.0f64..30.0,
        seconds in 0.0f64..6.0,
    ) {
        let damping = regime * 2.0 * (stiffness * mass).sqrt();
        let spring = Spring::new(mass, stiffness, damping, velocity).unwrap();
        let envelope = spring.motion.envelope(seconds);
        let displacement = spring.motion.displacement(seconds).abs();
        prop_assert!(envelope + 1e-9 >= displacement, "{envelope} < {displacement}");
    }

    #[test]
    #[allow(clippy::float_cmp)] // the ends of a curve are pinned exactly, by design
    fn has_a_finite_duration_and_stays_settled_after_it(mass in 0.1f64..10.0, stiffness in 10.0f64..1000.0, damping in 1.0f64..100.0, velocity in -20.0f64..20.0, extra in 0.0f64..1.0) {
        let spring = Spring::new(mass, stiffness, damping, velocity).unwrap();
        prop_assert!(spring.duration_ms() >= 1.0 && spring.duration_ms().is_finite());
        prop_assert_eq!(spring.at(spring.duration_ms() * (1.0 + extra)), 1.0);
    }

    #[test]
    fn the_curve_is_always_finite(mass in 0.1f64..10.0, stiffness in 10.0f64..1000.0, damping in 1.0f64..100.0, t in 0.0f64..=1.0) {
        prop_assert!(Spring::new(mass, stiffness, damping, 0.0).is_some_and(|s| s.easing(t).is_finite()));
    }
}
