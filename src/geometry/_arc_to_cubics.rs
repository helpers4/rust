// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Elliptical arcs to cubic Bézier curves, for [`normalize_path`](super::_normalize_path). Not
//! re-exported.

use super::PathCommand;
use std::f64::consts::PI;

/// An arc (SVG endpoint form, implementation notes F.6.5) as cubic `C` commands of at most a
/// quarter turn each. Radii that are too small are scaled up so the arc reaches its end point.
/// The caller handles the degenerate cases (a zero radius, equal end points).
#[allow(clippy::too_many_arguments, clippy::similar_names)]
pub(crate) fn arc_to_cubics(
    from: (f64, f64),
    radii: (f64, f64),
    rotation: f64,
    large_arc: bool,
    sweep: bool,
    to: (f64, f64),
) -> Vec<PathCommand> {
    let (x1, y1) = from;
    let (x2, y2) = to;
    let (mut rx, mut ry) = (radii.0.abs(), radii.1.abs());
    let phi = rotation.to_radians();
    let (sin, cos) = phi.sin_cos();
    let dx = (x1 - x2) / 2.0;
    let dy = (y1 - y2) / 2.0;
    let x1p = cos * dx + sin * dy;
    let y1p = -sin * dx + cos * dy;
    let lambda = (x1p * x1p) / (rx * rx) + (y1p * y1p) / (ry * ry);
    if lambda > 1.0 {
        rx *= lambda.sqrt();
        ry *= lambda.sqrt();
    }
    let num = rx * rx * ry * ry - rx * rx * y1p * y1p - ry * ry * x1p * x1p;
    let den = rx * rx * y1p * y1p + ry * ry * x1p * x1p;
    let coef = if large_arc == sweep { -1.0 } else { 1.0 } * (num / den).max(0.0).sqrt();
    let cxp = coef * rx * y1p / ry;
    let cyp = -coef * ry * x1p / rx;
    let cx = cos * cxp - sin * cyp + f64::midpoint(x1, x2);
    let cy = sin * cxp + cos * cyp + f64::midpoint(y1, y2);
    let theta = ((y1p - cyp) / ry).atan2((x1p - cxp) / rx);
    let mut delta = ((-y1p - cyp) / ry).atan2((-x1p - cxp) / rx) - theta;
    if !sweep && delta > 0.0 {
        delta -= 2.0 * PI;
    }
    if sweep && delta < 0.0 {
        delta += 2.0 * PI;
    }

    // At most a quarter turn per curve; `count` is small and positive.
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let count = ((delta.abs() / (PI / 2.0) - 1e-9).ceil() as usize).max(1);
    #[allow(clippy::cast_precision_loss)]
    let step = delta / count as f64;
    let k = 4.0 / 3.0 * (step / 4.0).tan();
    let map = |ux: f64, uy: f64| {
        (
            cx + rx * cos * ux - ry * sin * uy,
            cy + rx * sin * ux + ry * cos * uy,
        )
    };
    (0..count)
        .map(|i| {
            #[allow(clippy::cast_precision_loss)]
            let a = theta + i as f64 * step;
            let b = a + step;
            let c1 = map(a.cos() - k * a.sin(), a.sin() + k * a.cos());
            let c2 = map(b.cos() + k * b.sin(), b.sin() - k * b.cos());
            let end = if i == count - 1 {
                (x2, y2)
            } else {
                map(b.cos(), b.sin())
            };
            PathCommand::new('C', vec![c1.0, c1.1, c2.0, c2.1, end.0, end.1])
        })
        .collect()
}
