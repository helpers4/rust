// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::Rgba;

/// The blend modes of the W3C Compositing and Blending specification, the same ones as CSS
/// `mix-blend-mode`. The last four ([`Hue`](Self::Hue), [`Saturation`](Self::Saturation),
/// [`Color`](Self::Color), [`Luminosity`](Self::Luminosity)) are non-separable: they mix the
/// three channels together instead of one channel at a time.
///
/// # Examples
///
/// ```
/// use helpers4::color::BlendMode;
///
/// assert_eq!(BlendMode::default(), BlendMode::Normal);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum BlendMode {
    /// The source replaces the backdrop (a plain alpha composite). The default.
    #[default]
    Normal,
    /// Multiplies the channels: always darker.
    Multiply,
    /// The inverse of [`Multiply`](Self::Multiply): always lighter.
    Screen,
    /// [`Multiply`](Self::Multiply) or [`Screen`](Self::Screen) depending on the backdrop.
    Overlay,
    /// The darker of the two, per channel.
    Darken,
    /// The lighter of the two, per channel.
    Lighten,
    /// Brightens the backdrop to reflect the source.
    ColorDodge,
    /// Darkens the backdrop to reflect the source.
    ColorBurn,
    /// [`Multiply`](Self::Multiply) or [`Screen`](Self::Screen) depending on the source.
    HardLight,
    /// A softer [`HardLight`](Self::HardLight).
    SoftLight,
    /// The absolute difference of the two.
    Difference,
    /// Like [`Difference`](Self::Difference), with less contrast.
    Exclusion,
    /// The hue of the source with the saturation and luminosity of the backdrop.
    Hue,
    /// The saturation of the source with the hue and luminosity of the backdrop.
    Saturation,
    /// The hue and saturation of the source with the luminosity of the backdrop.
    Color,
    /// The luminosity of the source with the hue and saturation of the backdrop.
    Luminosity,
}

type Triplet = [f64; 3];

fn hard_light(backdrop: f64, source: f64) -> f64 {
    if source <= 0.5 {
        backdrop * 2.0 * source
    } else {
        backdrop + (2.0 * source - 1.0) - backdrop * (2.0 * source - 1.0)
    }
}

fn separable(mode: BlendMode, cb: f64, cs: f64) -> f64 {
    match mode {
        BlendMode::Multiply => cb * cs,
        BlendMode::Screen => cb + cs - cb * cs,
        BlendMode::Overlay => hard_light(cs, cb),
        BlendMode::Darken => cb.min(cs),
        BlendMode::Lighten => cb.max(cs),
        BlendMode::ColorDodge => {
            if cb == 0.0 {
                0.0
            } else if cs >= 1.0 {
                1.0
            } else {
                (cb / (1.0 - cs)).min(1.0)
            }
        }
        BlendMode::ColorBurn => {
            if cb >= 1.0 {
                1.0
            } else if cs == 0.0 {
                0.0
            } else {
                1.0 - ((1.0 - cb) / cs).min(1.0)
            }
        }
        BlendMode::HardLight => hard_light(cb, cs),
        BlendMode::SoftLight => {
            if cs <= 0.5 {
                cb - (1.0 - 2.0 * cs) * cb * (1.0 - cb)
            } else {
                let d = if cb <= 0.25 {
                    ((16.0 * cb - 12.0) * cb + 4.0) * cb
                } else {
                    cb.sqrt()
                };
                cb + (2.0 * cs - 1.0) * (d - cb)
            }
        }
        BlendMode::Difference => (cb - cs).abs(),
        BlendMode::Exclusion => cb + cs - 2.0 * cb * cs,
        _ => cs,
    }
}

fn lum([r, g, b]: Triplet) -> f64 {
    0.3 * r + 0.59 * g + 0.11 * b
}

fn extremes(c: Triplet) -> (f64, f64) {
    (c[0].min(c[1]).min(c[2]), c[0].max(c[1]).max(c[2]))
}

fn sat(c: Triplet) -> f64 {
    let (min, max) = extremes(c);
    max - min
}

fn clip_color(c: Triplet) -> Triplet {
    let l = lum(c);
    let (n, x) = extremes(c);
    c.map(|v| {
        let low = if n < 0.0 {
            l + (v - l) * l / (l - n)
        } else {
            v
        };
        if x > 1.0 {
            l + (low - l) * (1.0 - l) / (x - l)
        } else {
            low
        }
    })
}

fn set_lum(c: Triplet, l: f64) -> Triplet {
    let d = l - lum(c);
    clip_color(c.map(|v| v + d))
}

fn set_sat(c: Triplet, s: f64) -> Triplet {
    let (min, max) = extremes(c);
    if max <= min {
        return [0.0; 3];
    }
    c.map(|v| (v - min) * s / (max - min))
}

fn non_separable(mode: BlendMode, cb: Triplet, cs: Triplet) -> Triplet {
    match mode {
        BlendMode::Hue => set_lum(set_sat(cs, sat(cb)), lum(cb)),
        BlendMode::Saturation => set_lum(set_sat(cb, sat(cs)), lum(cb)),
        BlendMode::Color => set_lum(cs, lum(cb)),
        _ => set_lum(cb, lum(cs)),
    }
}

/// The channels as `0.0..=1.0` intensities.
fn unit(color: Rgba) -> Triplet {
    [color.r(), color.g(), color.b()].map(|c| f64::from(c) / 255.0)
}

/// Composites a source color over a backdrop with a blend mode, as the browser does for
/// `mix-blend-mode` (W3C Compositing and Blending Level 1, source-over). Both colors are straight
/// (non-premultiplied); the maths run in premultiplied space and the result is straight again,
/// each channel and the alpha rounded to the nearest 8-bit value.
///
/// # Arguments
///
/// - `backdrop` - The color underneath.
/// - `source` - The color painted over it.
/// - `mode` - The blend mode; [`BlendMode::Normal`] is a plain alpha composite.
///
/// # Returns
///
/// The straight result. Transparent black when both colors are fully transparent.
///
/// # Examples
///
/// ```
/// use helpers4::color::{blend, BlendMode, Rgba};
///
/// let red = Rgba::new(255, 0, 0, 255);
/// let half_blue = Rgba::new(0, 0, 255, 128);
/// assert_eq!(blend(red, half_blue, BlendMode::Normal), Rgba::new(127, 0, 128, 255));
///
/// let orange = Rgba::new(200, 100, 50, 255);
/// let gray = Rgba::new(100, 100, 100, 255);
/// assert_eq!(blend(orange, gray, BlendMode::Multiply), Rgba::new(78, 39, 20, 255));
/// ```
#[must_use]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // clamped to 0..=255 first
pub fn blend(backdrop: Rgba, source: Rgba, mode: BlendMode) -> Rgba {
    let ab = backdrop.opacity();
    let a_s = source.opacity();
    let ar = a_s + ab * (1.0 - a_s);
    if ar == 0.0 {
        return Rgba::new(0, 0, 0, 0);
    }
    let cb = unit(backdrop);
    let cs = unit(source);
    let blended: Triplet = match mode {
        BlendMode::Hue | BlendMode::Saturation | BlendMode::Color | BlendMode::Luminosity => {
            non_separable(mode, cb, cs)
        }
        _ => [0, 1, 2].map(|i| separable(mode, cb[i], cs[i])),
    };
    let channel = |i: usize| {
        let mixed = (1.0 - ab) * cs[i] + ab * blended[i];
        let value = (1.0 - a_s / ar) * cb[i] + (a_s / ar) * mixed;
        (value * 255.0).round().clamp(0.0, 255.0) as u8
    };
    Rgba::new(
        channel(0),
        channel(1),
        channel(2),
        (ar * 255.0).round().clamp(0.0, 255.0) as u8,
    )
}

#[cfg(test)]
#[path = "blend.test.rs"]
mod tests;

#[cfg(test)]
#[path = "blend.spec.rs"]
mod spec;
