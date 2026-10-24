// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Randomness without a hidden global generator.
//!
//! [`Rng`] is a small seedable generator for work that must be reproducible and testable:
//! [`shuffle`], [`sample`] and [`choose`] take it as an argument, so the same seed always gives
//! the same result. It is **not** for secrets: a 64-bit seed can be guessed.
//!
//! Keys, tokens and anything an attacker must not predict come from the operating system, with
//! the `random-entropy` feature (not in `default`, it pulls the `getrandom` crate):
//! [`random_bytes`], [`random_hex`], [`random_alphanumeric`] and [`Rng::from_entropy`]. They
//! return an [`EntropyError`] when the system cannot provide bytes, and never fall back to
//! weaker randomness. They are not built for `wasm32-unknown-unknown`, which has no default
//! entropy source: pass bytes from the host (`crypto.getRandomValues`) to the pure constructors
//! of the `id` module instead.

#[cfg(all(
    feature = "random-entropy",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
pub(crate) mod _fill_os;
mod choose;
#[cfg(all(
    feature = "random-entropy",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
mod entropy_error;
#[cfg(all(
    feature = "random-entropy",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
mod random_alphanumeric;
#[cfg(all(
    feature = "random-entropy",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
mod random_bytes;
#[cfg(all(
    feature = "random-entropy",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
mod random_hex;
mod rng;
mod sample;
mod shuffle;

pub use choose::choose;
#[cfg(all(
    feature = "random-entropy",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
pub use entropy_error::EntropyError;
#[cfg(all(
    feature = "random-entropy",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
pub use random_alphanumeric::random_alphanumeric;
#[cfg(all(
    feature = "random-entropy",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
pub use random_bytes::random_bytes;
#[cfg(all(
    feature = "random-entropy",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
pub use random_hex::random_hex;
pub use rng::Rng;
pub use sample::sample;
pub use shuffle::shuffle;
