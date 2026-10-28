// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::{FontFace, FontRequest, FontStyle};

fn ascending(values: &[f64], keep: impl Fn(f64) -> bool) -> Vec<f64> {
    let mut kept: Vec<f64> = values.iter().copied().filter(|v| keep(*v)).collect();
    kept.sort_by(f64::total_cmp);
    kept
}

fn descending(values: &[f64], keep: impl Fn(f64) -> bool) -> Vec<f64> {
    let mut kept = ascending(values, keep);
    kept.reverse();
    kept
}

/// Stretch values in the order CSS tries them.
fn stretch_order(wanted: f64, values: &[f64]) -> Vec<f64> {
    if wanted <= 100.0 {
        let mut order = descending(values, |v| v <= wanted);
        order.extend(ascending(values, |v| v > wanted));
        order
    } else {
        let mut order = ascending(values, |v| v >= wanted);
        order.extend(descending(values, |v| v < wanted));
        order
    }
}

/// Weight values in the order CSS tries them.
fn weight_order(wanted: f64, values: &[f64]) -> Vec<f64> {
    if (400.0..=500.0).contains(&wanted) {
        let mut order = ascending(values, |v| v >= wanted && v <= 500.0);
        order.extend(descending(values, |v| v < wanted));
        order.extend(ascending(values, |v| v > 500.0));
        order
    } else if wanted < 400.0 {
        let mut order = descending(values, |v| v <= wanted);
        order.extend(ascending(values, |v| v > wanted));
        order
    } else {
        let mut order = ascending(values, |v| v >= wanted);
        order.extend(descending(values, |v| v < wanted));
        order
    }
}

const fn style_order(wanted: FontStyle) -> [FontStyle; 3] {
    match wanted {
        FontStyle::Normal => [FontStyle::Normal, FontStyle::Oblique, FontStyle::Italic],
        FontStyle::Italic => [FontStyle::Italic, FontStyle::Oblique, FontStyle::Normal],
        FontStyle::Oblique => [FontStyle::Oblique, FontStyle::Italic, FontStyle::Normal],
    }
}

/// The faces of `pool` holding the first of `order` that any face has.
fn keep_first<T: Copy>(
    pool: &[usize],
    read: impl Fn(usize) -> T,
    order: &[T],
    same: impl Fn(T, T) -> bool,
) -> Vec<usize> {
    let best = order
        .iter()
        .copied()
        .find(|value| pool.iter().any(|&face| same(read(face), *value)));
    pool.iter()
        .copied()
        .filter(|&face| best.is_some_and(|value| same(read(face), value)))
        .collect()
}

/// The weight a face offers for a request: the requested weight when the face's range covers it,
/// otherwise the bound of the range closest to it.
fn offered_weight(face: &FontFace, wanted: f64) -> f64 {
    wanted.clamp(face.weight_min, face.weight_max.max(face.weight_min))
}

/// Picks the face that CSS would use for a request, among the faces of one family: the CSS Fonts
/// Level 4 matching algorithm (§5.2), as a pure function. The stretch is matched first, then the
/// style, then the weight; each step keeps only the faces closest to what was asked.
///
/// - Stretch: at or below 100, narrower faces first (nearest), then wider ones; above 100, the
///   other way round.
/// - Style: normal prefers normal, then oblique, then italic; italic prefers italic, then
///   oblique, then normal; oblique prefers oblique, then italic, then normal.
/// - Weight: a variable face whose range covers the weight is an exact match. Otherwise, from 400
///   to 500, the nearest heavier weight up to 500, then lighter ones, then heavier ones above
///   500; below 400, lighter first then heavier; above 500, heavier first then lighter. A
///   variable face counts for the bound of its range closest to the weight.
///
/// # Arguments
///
/// - `faces` - The faces of one family.
/// - `wanted` - What was asked.
///
/// # Returns
///
/// The index of the closest face, the first one when several are equally close, or `None` for no
/// face.
///
/// # Examples
///
/// ```
/// use helpers4::font::{pick_closest, FontFace, FontRequest, FontStyle};
///
/// let faces = [
///     FontFace::new(300.0, FontStyle::Normal),
///     FontFace::new(400.0, FontStyle::Normal),
///     FontFace::new(700.0, FontStyle::Normal),
/// ];
/// assert_eq!(pick_closest(&faces, FontRequest::new(600.0, FontStyle::Normal)), Some(2));
///
/// let variable = [FontFace::variable(100.0, 900.0, FontStyle::Normal), FontFace::new(600.0, FontStyle::Normal)];
/// assert_eq!(pick_closest(&variable, FontRequest::new(450.0, FontStyle::Normal)), Some(0));
/// ```
#[must_use]
pub fn pick_closest(faces: &[FontFace], wanted: FontRequest) -> Option<usize> {
    let all: Vec<usize> = (0..faces.len()).collect();
    let same_number = |a: f64, b: f64| a.total_cmp(&b).is_eq();

    let stretches: Vec<f64> = faces.iter().map(|face| face.stretch).collect();
    let pool = keep_first(
        &all,
        |i| faces[i].stretch,
        &stretch_order(wanted.stretch, &stretches),
        same_number,
    );
    let pool = keep_first(
        &pool,
        |i| faces[i].style,
        &style_order(wanted.style),
        |a, b| a == b,
    );
    let weights: Vec<f64> = pool
        .iter()
        .map(|&i| offered_weight(&faces[i], wanted.weight))
        .collect();
    let pool = keep_first(
        &pool,
        |i| offered_weight(&faces[i], wanted.weight),
        &weight_order(wanted.weight, &weights),
        same_number,
    );
    pool.first().copied()
}

#[cfg(test)]
#[path = "pick_closest.test.rs"]
mod tests;

#[cfg(test)]
#[path = "pick_closest.spec.rs"]
mod spec;
