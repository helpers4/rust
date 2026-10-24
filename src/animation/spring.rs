// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// The progress below which a spring is considered at rest, as a distance to its target.
const REST: f64 = 0.001;

/// A damped spring going from `0.0` to `1.0`: the curve behind "spring" transitions of design
/// tools, with the time it takes to settle.
///
/// The motion is solved exactly (under-, critically or over-damped), not by stepping a
/// simulation, so [`at`](Self::at) can be called at any time, in any order.
///
/// # Examples
///
/// ```
/// use helpers4::animation::Spring;
///
/// let bouncy = Spring::new(1.0, 300.0, 15.0, 0.0).expect("valid physics");
/// assert_eq!(bouncy.at(0.0), 0.0);
/// assert_eq!(bouncy.at(bouncy.duration_ms()), 1.0); // settled
/// assert!(bouncy.at(bouncy.duration_ms() * 0.3) > 1.0); // overshoots on the way
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Spring {
    motion: Motion,
    duration_ms: f64,
}

/// The exact solutions of the spring equation, by damping regime.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Motion {
    /// Oscillates while decaying: `e^(-decay·s) · (cos(w·s) + b·sin(w·s))`.
    Under { decay: f64, omega: f64, b: f64 },
    /// The fastest decay without oscillation: `(1 + b·s) · e^(-w0·s)`.
    Critical { w0: f64, b: f64 },
    /// Two decays, a slow and a fast one: `c1·e^(slow·s) + c2·e^(fast·s)`.
    Over {
        slow: f64,
        fast: f64,
        c1: f64,
        c2: f64,
    },
}

impl Motion {
    /// The distance to the target after `s` seconds: `1.0` at the start, `0.0` at rest.
    fn displacement(self, s: f64) -> f64 {
        match self {
            Self::Under { decay, omega, b } => {
                (-decay * s).exp() * ((omega * s).cos() + b * (omega * s).sin())
            }
            Self::Critical { w0, b } => (1.0 + b * s) * (-w0 * s).exp(),
            Self::Over { slow, fast, c1, c2 } => c1 * (slow * s).exp() + c2 * (fast * s).exp(),
        }
    }

    /// An upper bound of the absolute displacement that only decreases once it is below 1.
    fn envelope(self, s: f64) -> f64 {
        match self {
            Self::Under { decay, b, .. } => (-decay * s).exp() * 1.0f64.hypot(b),
            Self::Critical { w0, b } => (1.0 + b.abs() * s) * (-w0 * s).exp(),
            Self::Over { slow, c1, c2, .. } => (c1.abs() + c2.abs()) * (slow * s).exp(),
        }
    }
}

impl Spring {
    /// A spring from its physical parameters, as design tools (Figma, Framer, …) expose them.
    ///
    /// # Arguments
    ///
    /// - `mass` - Mass of the moving object, `> 0`. `1.0` is typical.
    /// - `stiffness` - Stiffness of the spring, `> 0`: higher is faster. `100.0` is typical.
    /// - `damping` - Friction, `> 0`: higher oscillates less. `10.0` is typical.
    /// - `initial_velocity` - Starting speed toward the target, in progress per second.
    ///
    /// # Returns
    ///
    /// The spring, or `None` when a parameter is not finite, or `mass`, `stiffness` or `damping`
    /// is not positive, or the spring never settles within a finite time.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::animation::Spring;
    ///
    /// assert!(Spring::new(1.0, 100.0, 10.0, 0.0).is_some());
    /// assert!(Spring::new(0.0, 100.0, 10.0, 0.0).is_none());
    /// ```
    #[must_use]
    pub fn new(mass: f64, stiffness: f64, damping: f64, initial_velocity: f64) -> Option<Self> {
        let valid = mass > 0.0
            && stiffness > 0.0
            && damping > 0.0
            && initial_velocity.is_finite()
            && (mass * stiffness * damping).is_finite();
        if !valid {
            return None;
        }
        let w0 = (stiffness / mass).sqrt();
        let zeta = damping / (2.0 * (stiffness * mass).sqrt());
        let motion = if (zeta - 1.0).abs() < 1e-9 {
            Motion::Critical {
                w0,
                b: w0 - initial_velocity,
            }
        } else if zeta < 1.0 {
            let omega = w0 * (1.0 - zeta * zeta).sqrt();
            Motion::Under {
                decay: zeta * w0,
                omega,
                b: (zeta * w0 - initial_velocity) / omega,
            }
        } else {
            let root = w0 * (zeta * zeta - 1.0).sqrt();
            let slow = -zeta * w0 + root;
            let fast = -zeta * w0 - root;
            let c2 = (-initial_velocity - slow) / (fast - slow);
            Motion::Over {
                slow,
                fast,
                c1: 1.0 - c2,
                c2,
            }
        };
        settling_time_ms(motion).map(|duration_ms| Self {
            motion,
            duration_ms,
        })
    }

    /// The time until the spring settles, from which it stays within `0.001` of its target.
    ///
    /// # Returns
    ///
    /// Milliseconds, at least `1.0`.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::animation::Spring;
    ///
    /// let spring = Spring::default();
    /// assert!(spring.duration_ms() > 1000.0 && spring.duration_ms() < 1500.0);
    /// ```
    #[must_use]
    pub fn duration_ms(&self) -> f64 {
        self.duration_ms
    }

    /// The progress at a time: `0.0` at the start, exactly `1.0` from
    /// [`duration_ms`](Self::duration_ms) on, and possibly above `1.0` while oscillating.
    ///
    /// # Arguments
    ///
    /// - `time_ms` - Milliseconds since the start. A time before the start (or `NaN`) gives `0.0`.
    ///
    /// # Returns
    ///
    /// The progress.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::animation::Spring;
    ///
    /// let spring = Spring::default();
    /// assert_eq!(spring.at(-5.0), 0.0);
    /// assert_eq!(spring.at(10_000.0), 1.0);
    /// ```
    #[must_use]
    pub fn at(&self, time_ms: f64) -> f64 {
        if time_ms.is_nan() || time_ms <= 0.0 {
            0.0
        } else if time_ms >= self.duration_ms {
            1.0
        } else {
            1.0 - self.motion.displacement(time_ms / 1000.0)
        }
    }

    /// The same curve as an easing over `0.0..=1.0`: the fraction of
    /// [`duration_ms`](Self::duration_ms), for [`to_css_linear`](super::to_css_linear) or any
    /// animation that takes an easing function.
    ///
    /// # Arguments
    ///
    /// - `t` - The progress of time, `0.0..=1.0`.
    ///
    /// # Returns
    ///
    /// The progress.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::animation::Spring;
    ///
    /// let spring = Spring::default();
    /// assert_eq!(spring.easing(0.0), 0.0);
    /// assert_eq!(spring.easing(1.0), 1.0);
    /// ```
    #[must_use]
    pub fn easing(&self, t: f64) -> f64 {
        self.at(t * self.duration_ms)
    }
}

impl Default for Spring {
    /// Mass `1.0`, stiffness `100.0`, damping `10.0`, no initial velocity: a lightly bouncy
    /// spring that settles in about 1.3 seconds.
    fn default() -> Self {
        // The solution for these values (damping ratio 0.5), written out so that `default` cannot
        // fail; a test checks it equals `Spring::new(1.0, 100.0, 10.0, 0.0)`.
        let omega = 100.0f64.sqrt() * 0.75f64.sqrt();
        let motion = Motion::Under {
            decay: 5.0,
            omega,
            b: 5.0 / omega,
        };
        Self {
            motion,
            duration_ms: settling_time_ms(motion).unwrap_or(1.0),
        }
    }
}

/// When the motion is within [`REST`] of its target for good, in milliseconds.
fn settling_time_ms(motion: Motion) -> Option<f64> {
    // Double `end` until the envelope is below REST: 1024 doublings overflow, so a motion that has
    // not settled by then never does within a finite time.
    let mut end = 1.0f64;
    for _ in 0..1100 {
        if motion.envelope(end / 1000.0) < REST {
            break;
        }
        end *= 2.0;
    }
    if !end.is_finite() {
        return None;
    }
    // Past `end` the motion is at rest: walk back to the last moment it was not. The step keeps
    // this to at most 10 000 iterations.
    let step = (end / 10_000.0).ceil().max(1.0);
    let mut last = end;
    for _ in 0..=10_000 {
        if last <= 0.0 || motion.displacement(last / 1000.0).abs() >= REST {
            break;
        }
        last -= step;
    }
    Some((last + step).max(1.0))
}

#[cfg(test)]
#[path = "spring.test.rs"]
mod tests;

#[cfg(test)]
#[path = "spring.spec.rs"]
mod spec;
