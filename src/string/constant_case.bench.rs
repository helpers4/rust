// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::Criterion;
use helpers4::string::constant_case;
use std::hint::black_box;

pub fn bench(c: &mut Criterion) {
    c.bench_function("string::constant_case", |b| {
        b.iter(|| constant_case(black_box("someHTTPServerName here now")));
    });
}
