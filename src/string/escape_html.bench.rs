// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::Criterion;
use helpers4::string::escape_html;
use std::hint::black_box;

pub fn bench(c: &mut Criterion) {
    c.bench_function("string::escape_html/clean", |b| {
        b.iter(|| escape_html(black_box("nothing to escape here")));
    });
    c.bench_function("string::escape_html/dirty", |b| {
        b.iter(|| escape_html(black_box("<a href=\"x\">Tom & Jerry's</a>")));
    });
}
