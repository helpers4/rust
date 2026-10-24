// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Unique identifiers without the `uuid` or `ulid` crates: [`Uuid`] (v4 random, v7 sortable by
//! time), [`Ulid`], and short URL-safe ids over an [`Alphabet`].
//!
//! Randomness and time are **parameters**: `Uuid::v4(random_bytes)`, `Uuid::v7(unix_millis,
//! random_bytes)`, `Ulid::from_parts(unix_millis, random_bytes)` and `short_id(alphabet, len,
//! &mut rng)` are pure, so they are testable and work anywhere, `wasm32-unknown-unknown`
//! included. With the `random-entropy` feature, `Uuid::new_v4`, `Uuid::new_v7`, `Ulid::new` and
//! `new_short_id` take both from the operating system and the clock.
//!
//! Ids made with a seeded [`Rng`](crate::random::Rng) are reproducible, not secret.

#[cfg(all(
    feature = "random-entropy",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
mod _unix_millis;
mod alphabet;
mod error;
#[cfg(all(
    feature = "random-entropy",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
mod new_short_id;
mod short_id;
mod ulid;
mod uuid;

pub use alphabet::Alphabet;
pub use error::AlphabetError;
pub use error::ParseUlidError;
pub use error::ParseUuidError;
#[cfg(all(
    feature = "random-entropy",
    not(all(target_arch = "wasm32", target_os = "unknown"))
))]
pub use new_short_id::new_short_id;
pub use short_id::short_id;
pub use ulid::Ulid;
pub use uuid::Uuid;
