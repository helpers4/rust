// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::*;
use proptest::prelude::*;

fn entry() -> impl Strategy<Value = f64> {
    -10.0..10.0f64
}

fn matrix() -> impl Strategy<Value = Matrix> {
    (entry(), entry(), entry(), entry(), entry(), entry())
        .prop_map(|(a, b, c, d, e, f)| Matrix::new(a, b, c, d, e, f))
}

fn point() -> impl Strategy<Value = Point> {
    (-1000.0..1000.0f64, -1000.0..1000.0f64).prop_map(|(x, y)| Point::new(x, y))
}

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() <= 1e-6 * (1.0 + a.abs().max(b.abs()))
}

proptest! {
    #[test]
    fn multiply_composes_apply(m in matrix(), n in matrix(), p in point()) {
        let once = m.multiply(n).apply(p);
        let twice = m.apply(n.apply(p));
        prop_assert!(close(once.x, twice.x) && close(once.y, twice.y));
    }

    #[test]
    fn invert_brings_points_back(angle in -10.0..10.0f64, sx in 0.1..10.0f64, sy in 0.1..10.0f64, tx in entry(), ty in entry(), p in point()) {
        let (sin, cos) = angle.sin_cos();
        let m = Matrix::new(cos * sx, sin * sx, -sin * sy, cos * sy, tx, ty);
        let back = m.invert().map(|inv| inv.apply(m.apply(p)));
        prop_assert!(back.is_some_and(|q| close(q.x, p.x) && close(q.y, p.y)));
    }

    #[test]
    fn bounding_box_contains_the_corners(m in matrix(), x in -100.0..100.0f64, y in -100.0..100.0f64, w in 0.0..100.0f64, h in 0.0..100.0f64) {
        let rect = Rect::new(x, y, w, h);
        let b = m.bounding_box(rect);
        prop_assert!(b.width >= 0.0 && b.height >= 0.0);
        for corner in [Point::new(x, y), Point::new(x + w, y + h)] {
            let q = m.apply(corner);
            prop_assert!(q.x >= b.x - 1e-9 && q.x <= b.right() + 1e-9);
            prop_assert!(q.y >= b.y - 1e-9 && q.y <= b.bottom() + 1e-9);
        }
    }
}
