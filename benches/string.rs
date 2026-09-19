// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::{Criterion, criterion_group, criterion_main};
use helpers4::string::capitalize;
use std::hint::black_box;

fn bench_capitalize(c: &mut Criterion) {
    c.bench_function("string::capitalize", |b| {
        b.iter(|| capitalize(black_box("hello world")))
    });
}

criterion_group!(benches, bench_capitalize);
criterion_main!(benches);
