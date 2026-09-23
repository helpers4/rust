// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::Criterion;
use helpers4::number::median;
use std::hint::black_box;

pub fn bench(c: &mut Criterion) {
    let values: Vec<f64> = (0..1000).map(|i| f64::from(i % 97) * 1.5).collect();
    c.bench_function("number::median", |b| {
        b.iter(|| median(black_box(&values)));
    });
}
