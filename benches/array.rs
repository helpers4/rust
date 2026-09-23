// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// `criterion_group!` and `criterion_main!` generate public, undocumented items.
#![allow(missing_docs, unreachable_pub)]

use criterion::{criterion_group, criterion_main};

#[path = "../src/array/difference.bench.rs"]
mod difference;
#[path = "../src/array/duplicates.bench.rs"]
mod duplicates;
#[path = "../src/array/equals_unordered.bench.rs"]
mod equals_unordered;
#[path = "../src/array/group_by.bench.rs"]
mod group_by;
#[path = "../src/array/interleave.bench.rs"]
mod interleave;
#[path = "../src/array/key_by.bench.rs"]
mod key_by;
#[path = "../src/array/unique.bench.rs"]
mod unique;

criterion_group!(
    benches,
    difference::bench,
    duplicates::bench,
    equals_unordered::bench,
    group_by::bench,
    interleave::bench,
    key_by::bench,
    unique::bench,
);
criterion_main!(benches);
