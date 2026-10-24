// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

//! Animation timing without a framework: [`Easing`] curves (the CSS keywords, `cubic-bezier()`,
//! `steps()` and `linear()`, read from a CSS string or built directly), physical [`Spring`]s with
//! their settling time, and [`to_css_linear`] to turn any curve into a CSS `linear()`.
//!
//! The same model as the TypeScript package's `animation` category, for code that computes
//! animation values itself (a renderer, a native player, a tool that exports animations).

mod _parse;
mod easing;
mod error;
mod spring;
mod step_position;
mod to_css_linear;

pub use easing::Easing;
pub use error::ParseEasingError;
pub use spring::Spring;
pub use step_position::StepPosition;
pub use to_css_linear::to_css_linear;
