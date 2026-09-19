// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::Criterion;
use helpers4::array::equals_unordered;
use std::hint::black_box;

pub fn bench(c: &mut Criterion) {
    let items: Vec<u32> = (0..1000).map(|i| i % 97).collect();
    let mut shuffled = items.clone();
    shuffled.reverse();
    c.bench_function("array::equals_unordered", |b| {
        b.iter(|| equals_unordered(black_box(&items), black_box(&shuffled)));
    });
}
