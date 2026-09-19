// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use criterion::{criterion_group, criterion_main};

#[path = "../src/hex/decode.bench.rs"]
mod decode;
#[path = "../src/hex/encode.bench.rs"]
mod encode;

criterion_group!(benches, decode::bench, encode::bench);
criterion_main!(benches);
