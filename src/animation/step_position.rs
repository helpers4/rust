// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

/// Where the jumps of a `steps()` easing happen, as in CSS.
///
/// # Examples
///
/// ```
/// use helpers4::animation::{Easing, StepPosition};
///
/// let four_frames = Easing::steps(4, StepPosition::JumpEnd);
/// assert_eq!(four_frames.map(|e| e.at(0.3)), Some(0.25));
/// let jump_first = Easing::steps(4, StepPosition::JumpStart);
/// assert_eq!(jump_first.map(|e| e.at(0.3)), Some(0.5));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum StepPosition {
    /// The first jump happens at the start (CSS `jump-start`, or `start`).
    JumpStart,
    /// The last jump happens at the end, so the first value lasts a whole step (CSS `jump-end`,
    /// or `end`). The default.
    #[default]
    JumpEnd,
    /// No jump at either end: `0` and `1` are each held for a step (CSS `jump-none`).
    JumpNone,
    /// A jump at both ends, so neither `0` nor `1` is held (CSS `jump-both`).
    JumpBoth,
}
