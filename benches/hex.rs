// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

// `criterion_group!` and `criterion_main!` generate public, undocumented items.
#![allow(missing_docs, unreachable_pub)]

use criterion::{criterion_group, criterion_main};

#[path = "../src/hex/decode.bench.rs"]
mod decode;
#[path = "../src/hex/encode.bench.rs"]
mod encode;

criterion_group!(benches, decode::bench, encode::bench);
criterion_main!(benches);
