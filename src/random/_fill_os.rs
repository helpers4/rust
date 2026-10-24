// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::EntropyError;

/// Fills `dest` with bytes from the operating system's random source.
pub(crate) fn fill_os(dest: &mut [u8]) -> Result<(), EntropyError> {
    getrandom::fill(dest).map_err(EntropyError::from)
}

#[cfg(test)]
#[path = "_fill_os.test.rs"]
mod tests;
