// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::{Criterion, criterion_group, criterion_main};
use helpers4::string::{camel_case, capitalize, dedent, escape_html, slugify, snake_case};
use std::hint::black_box;

fn bench_capitalize(c: &mut Criterion) {
    c.bench_function("string::capitalize", |b| {
        b.iter(|| capitalize(black_box("hello world")))
    });
}

fn bench_case_conversion(c: &mut Criterion) {
    c.bench_function("string::camel_case", |b| {
        b.iter(|| camel_case(black_box("some_HTTPServer-name here")))
    });
    c.bench_function("string::snake_case", |b| {
        b.iter(|| snake_case(black_box("someHTTPServerName here")))
    });
}

fn bench_slugify(c: &mut Criterion) {
    c.bench_function("string::slugify", |b| {
        b.iter(|| slugify(black_box("  It's a Fine Day -- isn't it?  ")))
    });
}

fn bench_escape_html(c: &mut Criterion) {
    c.bench_function("string::escape_html/clean", |b| {
        b.iter(|| escape_html(black_box("nothing to escape here")))
    });
    c.bench_function("string::escape_html/dirty", |b| {
        b.iter(|| escape_html(black_box("<a href=\"x\">Tom & Jerry's</a>")))
    });
}

fn bench_dedent(c: &mut Criterion) {
    c.bench_function("string::dedent", |b| {
        b.iter(|| dedent(black_box("\n    Hello\n      World\n    !\n")))
    });
}

criterion_group!(
    benches,
    bench_capitalize,
    bench_case_conversion,
    bench_slugify,
    bench_escape_html,
    bench_dedent
);
criterion_main!(benches);
