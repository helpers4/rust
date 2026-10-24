#!/bin/bash -eu
# This file is part of helpers4.
# Copyright (C) 2025 baxyz
# SPDX-License-Identifier: LGPL-3.0-or-later

# Builds every cargo-fuzz target of fuzz/ and copies the binaries to $OUT, where ClusterFuzzLite
# looks for them.
cd "$SRC/rust"
cargo fuzz build -O

TARGET_DIR="fuzz/target/x86_64-unknown-linux-gnu/release"
for source in fuzz/fuzz_targets/*.rs; do
  name="$(basename "${source%.rs}")"
  cp "$TARGET_DIR/$name" "$OUT/"
done
