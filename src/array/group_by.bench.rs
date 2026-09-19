// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::Criterion;
use helpers4::array::group_by;
use std::hint::black_box;

pub fn bench(c: &mut Criterion) {
    let items: Vec<u32> = (0..1000).map(|i| i % 97).collect();
    c.bench_function("array::group_by", |b| {
        b.iter(|| group_by(black_box(&items), |x| x % 10))
    });
}
