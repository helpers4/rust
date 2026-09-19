// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::Criterion;
use helpers4::hex::encode;
use std::hint::black_box;

pub fn bench(c: &mut Criterion) {
    let key = [0xa5u8; 32];
    c.bench_function("hex::encode", |b| b.iter(|| encode(black_box(&key))));
}
