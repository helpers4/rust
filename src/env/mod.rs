// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Dotenv (`.env`) helpers working on plain text: no file or process-environment access, so
//! they are deterministic and easy to test. Read the file yourself, edit the content here, and
//! write it back.

mod _line;
mod get;
mod parse;

pub use get::get;
pub use parse::parse;
