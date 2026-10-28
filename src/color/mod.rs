// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Colors without a dependency: parsing, hex, CSS output, HSL, alpha, blending and WCAG contrast.
//!
//! [`Rgb`] is an 8-bit sRGB color that parses from `#rgb`, `#rrggbb`, `rgb(r, g, b)` or a basic CSS name;
//! [`Hsl`] converts to and from it; `mix` blends two colors; `contrast_ratio` and `best_text_color` answer
//! "is this text readable on that background".

mod _named;
mod best_text_color;
mod blend;
mod contrast_ratio;
mod error;
mod hsb;
mod hsl;
mod mix;
mod premultiply;
mod rgb;
mod rgba;
mod unpremultiply;

pub use best_text_color::best_text_color;
pub use blend::BlendMode;
pub use blend::blend;
pub use contrast_ratio::contrast_ratio;
pub use error::ParseColorError;
pub use hsb::Hsb;
pub use hsl::Hsl;
pub use mix::mix;
pub use premultiply::premultiply;
pub use rgb::Rgb;
pub use rgba::Rgba;
pub use unpremultiply::unpremultiply;
