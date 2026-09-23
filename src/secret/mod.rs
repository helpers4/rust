// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Keeping secrets out of logs: a wrapper that never prints its value, redaction, masking and
//! credential detection.
//!
//! [`Secret`] hides a value from `Debug` and `Display`; `redact` and `mask` clean text you already have;
//! `detect` and `scan` recognize well-known token formats (GitHub, AWS, Slack, Stripe, Google, npm, JWT,
//! PEM keys). None of it wipes memory (that needs unsafe code, which this crate forbids).

mod detect;
mod mask;
mod redact;
mod scan;
mod secret_value;
mod token_kind;

pub use detect::detect;
pub use mask::mask;
pub use redact::REDACTED;
pub use redact::redact;
pub use scan::Finding;
pub use scan::scan;
pub use secret_value::Secret;
pub use token_kind::TokenKind;
