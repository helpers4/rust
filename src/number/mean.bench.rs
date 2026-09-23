// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::Criterion;
use helpers4::number::mean;
use std::hint::black_box;

pub fn bench(c: &mut Criterion) {
    let values: Vec<f64> = (0..1000).map(|i| f64::from(i % 97) * 1.5).collect();
    c.bench_function("number::mean", |b| {
        b.iter(|| mean(black_box(&values)));
    });
}
