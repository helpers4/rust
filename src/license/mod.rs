// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! SPDX license identifiers and expressions: look a license up, parse `MIT OR Apache-2.0`, check it
//! against a policy, write a source header.
//!
//! [`lookup`] gives the name and family ([`Category`]) of about forty common licenses, `normalize`
//! updates the deprecated GNU identifiers, [`Expression`] parses and evaluates SPDX expressions, and
//! `header` writes the copyright and `SPDX-License-Identifier` lines. This is information for tooling,
//! not legal advice.

mod _table;
mod category;
mod error;
mod expression;
mod header;
mod license_info;
mod lookup;
mod normalize;

pub use category::Category;
pub use error::ParseExpressionError;
pub use expression::Expression;
pub use header::header;
pub use license_info::LicenseInfo;
pub use lookup::lookup;
pub use normalize::normalize;
