// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::Criterion;
use helpers4::string::capitalize;
use std::hint::black_box;

pub fn bench(c: &mut Criterion) {
    c.bench_function("string::capitalize", |b| {
        b.iter(|| capitalize(black_box("hello world")));
    });
}
