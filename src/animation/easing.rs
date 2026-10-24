// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use std::str::FromStr;

use super::{ParseEasingError, StepPosition};

/// A timing curve: how the progress of an animation follows the progress of time, from `0.0` to
/// `1.0`. The same curves as CSS `animation-timing-function`.
///
/// Build one directly ([`linear`](Self::linear), [`cubic_bezier`](Self::cubic_bezier),
/// [`steps`](Self::steps)) or read it from a CSS string, e.g. the easing a design tool exported
/// ([`parse`](Self::parse): keywords, `cubic-bezier()`, `steps()` and `linear()`), then evaluate
/// it with [`at`](Self::at).
///
/// # Examples
///
/// ```
/// use helpers4::animation::Easing;
///
/// let ease_out = Easing::parse("ease-out")?;
/// assert!((ease_out.at(0.5) - 0.6846).abs() < 1e-4);
///
/// let bouncy = Easing::parse("cubic-bezier(0.34, 1.56, 0.64, 1)")?;
/// assert!(bouncy.at(0.5) > 1.0); // overshoots before settling
/// # Ok::<(), helpers4::animation::ParseEasingError>(())
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Easing {
    kind: Kind,
}

#[derive(Debug, Clone, PartialEq)]
enum Kind {
    Linear,
    CubicBezier { x1: f64, y1: f64, x2: f64, y2: f64 },
    Steps { count: u32, position: StepPosition },
    Stops(Vec<Stop>),
}

/// A point of a CSS `linear()` easing: the progress of time and the output there.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Stop {
    pub(crate) input: f64,
    pub(crate) output: f64,
}

impl Easing {
    /// The identity: the animation progresses as fast as time.
    ///
    /// # Returns
    ///
    /// The easing.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::animation::Easing;
    ///
    /// assert_eq!(Easing::linear().at(0.25), 0.25);
    /// ```
    #[must_use]
    pub const fn linear() -> Self {
        Self { kind: Kind::Linear }
    }

    /// The curve of CSS `cubic-bezier(x1, y1, x2, y2)`. The `y` values may leave `0..=1` to
    /// overshoot; the `x` values may not, as in CSS.
    ///
    /// # Arguments
    ///
    /// - `x1` - x of the first control point, `0.0..=1.0`.
    /// - `y1` - y of the first control point.
    /// - `x2` - x of the second control point, `0.0..=1.0`.
    /// - `y2` - y of the second control point.
    ///
    /// # Returns
    ///
    /// The easing, or `None` when an `x` is outside `0.0..=1.0` or a value is not finite.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::animation::Easing;
    ///
    /// let ease = Easing::cubic_bezier(0.25, 0.1, 0.25, 1.0);
    /// assert!(ease.is_some_and(|e| (e.at(0.5) - 0.8024).abs() < 1e-4));
    /// assert_eq!(Easing::cubic_bezier(1.5, 0.0, 1.0, 1.0), None);
    /// ```
    #[must_use]
    pub fn cubic_bezier(x1: f64, y1: f64, x2: f64, y2: f64) -> Option<Self> {
        let valid = (0.0..=1.0).contains(&x1)
            && (0.0..=1.0).contains(&x2)
            && y1.is_finite()
            && y2.is_finite();
        valid.then(|| Self::cubic_bezier_unchecked(x1, y1, x2, y2))
    }

    /// The curve of CSS `cubic-bezier()` for values already known to be valid.
    pub(crate) const fn cubic_bezier_unchecked(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self {
            kind: Kind::CubicBezier { x1, y1, x2, y2 },
        }
    }

    /// The curve of CSS `steps(count, position)`: the progress moves by jumps instead of
    /// continuously, for frame-by-frame sprites, ticking counters or an "instant" transition.
    ///
    /// # Arguments
    ///
    /// - `count` - Number of steps, at least `1` (at least `2` with [`StepPosition::JumpNone`]).
    /// - `position` - Where the jumps happen.
    ///
    /// # Returns
    ///
    /// The easing, or `None` for a `count` that is too small.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::animation::{Easing, StepPosition};
    ///
    /// let frames = Easing::steps(4, StepPosition::JumpEnd);
    /// assert_eq!(frames.map(|e| e.at(0.6)), Some(0.5));
    /// assert_eq!(Easing::steps(1, StepPosition::JumpNone), None);
    /// ```
    #[must_use]
    pub fn steps(count: u32, position: StepPosition) -> Option<Self> {
        let minimum = if position == StepPosition::JumpNone {
            2
        } else {
            1
        };
        (count >= minimum).then_some(Self {
            kind: Kind::Steps { count, position },
        })
    }

    /// A CSS `linear()` easing from stops already resolved by the parser (at least two, inputs
    /// never decreasing).
    pub(crate) const fn from_stops(stops: Vec<Stop>) -> Self {
        Self {
            kind: Kind::Stops(stops),
        }
    }

    /// Reads a CSS easing: `linear`, `ease`, `ease-in`, `ease-out`, `ease-in-out`, `step-start`,
    /// `step-end`, `cubic-bezier(x1, y1, x2, y2)`, `steps(count[, position])` or
    /// `linear(0, 0.25 40%, 1)` (points with optional input positions, which CSS fills in).
    ///
    /// Case and extra spaces are ignored.
    ///
    /// # Arguments
    ///
    /// - `css` - The CSS easing.
    ///
    /// # Returns
    ///
    /// The easing.
    ///
    /// # Errors
    ///
    /// Returns a [`ParseEasingError`] for anything that is not a CSS easing, or a known function
    /// with arguments CSS rejects.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::animation::Easing;
    ///
    /// assert_eq!(Easing::parse("linear(0, 0.9 50%, 1)")?.at(0.25), 0.45);
    /// assert_eq!(Easing::parse("steps(4, jump-start)")?.at(0.3), 0.5);
    /// assert!(Easing::parse("bounce").is_err());
    /// # Ok::<(), helpers4::animation::ParseEasingError>(())
    /// ```
    pub fn parse(css: &str) -> Result<Self, ParseEasingError> {
        super::_parse::parse(css)
    }

    /// The progress of the animation at a progress of time.
    ///
    /// # Arguments
    ///
    /// - `t` - The progress of time. Clamped to `0.0..=1.0`; a `NaN` counts as `0.0`.
    ///
    /// # Returns
    ///
    /// The progress of the animation: `0.0` at the start and `1.0` at the end, possibly outside
    /// `0.0..=1.0` in between for curves that overshoot.
    ///
    /// # Examples
    ///
    /// ```
    /// use helpers4::animation::Easing;
    ///
    /// let ease = Easing::parse("ease-in-out")?;
    /// assert_eq!(ease.at(0.0), 0.0);
    /// assert!((ease.at(0.5) - 0.5).abs() < 1e-9);
    /// assert_eq!(ease.at(2.0), 1.0);
    /// # Ok::<(), helpers4::animation::ParseEasingError>(())
    /// ```
    #[must_use]
    pub fn at(&self, t: f64) -> f64 {
        let t = if t.is_nan() { 0.0 } else { t.clamp(0.0, 1.0) };
        match &self.kind {
            Kind::Linear => t,
            Kind::CubicBezier { x1, y1, x2, y2 } => cubic_bezier_at(*x1, *y1, *x2, *y2, t),
            Kind::Steps { count, position } => steps_at(*count, *position, t),
            Kind::Stops(stops) => stops_at(stops, t),
        }
    }
}

impl FromStr for Easing {
    type Err = ParseEasingError;

    fn from_str(css: &str) -> Result<Self, Self::Err> {
        Self::parse(css)
    }
}

fn cubic_bezier_at(x1: f64, y1: f64, x2: f64, y2: f64, t: f64) -> f64 {
    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }
    // Bernstein form expanded into a·s³ + b·s² + c·s, for both coordinates.
    let cx = 3.0 * x1;
    let bx = 3.0 * (x2 - x1) - cx;
    let ax = 1.0 - cx - bx;
    let cy = 3.0 * y1;
    let by = 3.0 * (y2 - y1) - cy;
    let ay = 1.0 - cy - by;
    let sample_x = |s: f64| ((ax * s + bx) * s + cx) * s;
    let sample_y = |s: f64| ((ay * s + by) * s + cy) * s;
    // x(s) never decreases on 0..=1 when x1 and x2 are in 0..=1, so bisection always converges:
    // 40 halvings narrow the interval to 2^-40, about 9e-13.
    let (mut low, mut high) = (0.0, 1.0);
    for _ in 0..40 {
        let middle = f64::midpoint(low, high);
        if sample_x(middle) < t {
            low = middle;
        } else {
            high = middle;
        }
    }
    sample_y(f64::midpoint(low, high))
}

fn steps_at(count: u32, position: StepPosition, t: f64) -> f64 {
    let jumps = match position {
        StepPosition::JumpBoth => count + 1,
        StepPosition::JumpNone => count - 1,
        StepPosition::JumpStart | StepPosition::JumpEnd => count,
    };
    let mut step = (t * f64::from(count)).floor();
    if matches!(position, StepPosition::JumpStart | StepPosition::JumpBoth) {
        step += 1.0;
    }
    step.min(f64::from(jumps)) / f64::from(jumps)
}

fn stops_at(stops: &[Stop], t: f64) -> f64 {
    let first = stops[0];
    let last = stops[stops.len() - 1];
    if t <= first.input {
        return first.output;
    }
    if t >= last.input {
        return last.output;
    }
    // The last stop at or before `t`; the next one is strictly after it.
    let mut index = 0;
    while stops[index + 1].input <= t {
        index += 1;
    }
    let (a, b) = (stops[index], stops[index + 1]);
    a.output + (b.output - a.output) * (t - a.input) / (b.input - a.input)
}

#[cfg(test)]
#[path = "easing.test.rs"]
mod tests;

#[cfg(test)]
#[path = "easing.spec.rs"]
mod spec;
