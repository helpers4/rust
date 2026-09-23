// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::Criterion;
use helpers4::string::indent;
use std::hint::black_box;

pub fn bench(c: &mut Criterion) {
    let text = "first line\nsecond line\n\nfourth line\nfifth line\n";
    c.bench_function("string::indent", |b| {
        b.iter(|| indent(black_box(text), black_box("    ")));
    });
}
