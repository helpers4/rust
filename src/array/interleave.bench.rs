// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::Criterion;
use helpers4::array::interleave;
use std::hint::black_box;

pub fn bench(c: &mut Criterion) {
    let first: Vec<u32> = (0..500).collect();
    let second: Vec<u32> = (500..1000).collect();
    c.bench_function("array::interleave", |b| {
        b.iter(|| interleave(black_box(&first), black_box(&second)));
    });
}
