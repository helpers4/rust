// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Shape checks for common user-facing formats: pragmatic subsets that catch real mistakes,
//! not full grammars. None of them normalizes or parses the value, only checks it.

mod is_slug;
mod is_uuid;
mod is_valid_email;

pub use is_slug::is_slug;
pub use is_uuid::is_uuid;
pub use is_valid_email::is_valid_email;
