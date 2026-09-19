// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::Criterion;
use helpers4::string::camel_case;
use std::hint::black_box;

pub fn bench(c: &mut Criterion) {
    c.bench_function("string::camel_case", |b| {
        b.iter(|| camel_case(black_box("some_HTTPServer-name here")));
    });
}
