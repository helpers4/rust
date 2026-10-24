// This file is part of helpers4.
// Copyright (C) 2025 baxyz
// SPDX-License-Identifier: LGPL-3.0-or-later

use super::easing::Stop;
use super::{Easing, ParseEasingError, StepPosition};

/// Reads a CSS easing. See [`Easing::parse`], the public wrapper around this.
pub(super) fn parse(css: &str) -> Result<Easing, ParseEasingError> {
    let value = css.trim().to_ascii_lowercase();
    match value.as_str() {
        "linear" => return Ok(Easing::linear()),
        "ease" => return Ok(Easing::cubic_bezier_unchecked(0.25, 0.1, 0.25, 1.0)),
        "ease-in" => return Ok(Easing::cubic_bezier_unchecked(0.42, 0.0, 1.0, 1.0)),
        "ease-out" => return Ok(Easing::cubic_bezier_unchecked(0.0, 0.0, 0.58, 1.0)),
        "ease-in-out" => return Ok(Easing::cubic_bezier_unchecked(0.42, 0.0, 0.58, 1.0)),
        "step-start" => return steps(1, StepPosition::JumpStart),
        "step-end" => return steps(1, StepPosition::JumpEnd),
        _ => {}
    }
    let (name, arguments) = split_call(&value).ok_or(ParseEasingError::UnknownFunction)?;
    let arguments: Vec<&str> = arguments.split(',').map(str::trim).collect();
    match name {
        "cubic-bezier" => parse_cubic_bezier(&arguments),
        "steps" => parse_steps(&arguments),
        "linear" => parse_linear(&arguments),
        _ => Err(ParseEasingError::UnknownFunction),
    }
}

/// `name(arguments)`, with no parentheses inside the arguments. The name is checked by the caller,
/// which only knows three functions.
fn split_call(value: &str) -> Option<(&str, &str)> {
    let open = value.find('(')?;
    let arguments = value[open + 1..].strip_suffix(')')?;
    (!arguments.contains(['(', ')'])).then_some((&value[..open], arguments))
}

/// A CSS number: optional sign, digits, a dot, an exponent. Not `inf` or `NaN`, which Rust reads.
fn number(text: &str) -> Option<f64> {
    let plain = text
        .chars()
        .all(|c| c.is_ascii_digit() || matches!(c, '.' | 'e' | '+' | '-'));
    plain
        .then(|| text.parse::<f64>().ok())
        .flatten()
        .filter(|value| value.is_finite())
}

fn steps(count: u32, position: StepPosition) -> Result<Easing, ParseEasingError> {
    Easing::steps(count, position).ok_or(ParseEasingError::InvalidArguments)
}

fn parse_cubic_bezier(arguments: &[&str]) -> Result<Easing, ParseEasingError> {
    let numbers: Option<Vec<f64>> = arguments.iter().map(|argument| number(argument)).collect();
    match numbers.as_deref() {
        Some(&[x1, y1, x2, y2]) => {
            Easing::cubic_bezier(x1, y1, x2, y2).ok_or(ParseEasingError::InvalidArguments)
        }
        _ => Err(ParseEasingError::InvalidArguments),
    }
}

fn parse_steps(arguments: &[&str]) -> Result<Easing, ParseEasingError> {
    // A CSS integer: optional `+`, then digits (no `-`, no decimals, no exponent).
    let count = arguments.first().and_then(|count| {
        let digits = count.strip_prefix('+').unwrap_or(count);
        digits
            .chars()
            .all(|c| c.is_ascii_digit())
            .then(|| digits.parse::<u32>().ok())
            .flatten()
    });
    let position = match arguments.get(1).copied() {
        Some("jump-start" | "start") => Some(StepPosition::JumpStart),
        None | Some("jump-end" | "end") => Some(StepPosition::JumpEnd),
        Some("jump-none") => Some(StepPosition::JumpNone),
        Some("jump-both") => Some(StepPosition::JumpBoth),
        Some(_) => None,
    };
    match (count, position, arguments.len()) {
        (Some(count), Some(position), 1 | 2) => steps(count, position),
        _ => Err(ParseEasingError::InvalidArguments),
    }
}

/// A `linear()` stop being read: an output, and its input position if the text gave one.
struct RawStop {
    input: Option<f64>,
    output: f64,
}

fn parse_linear(arguments: &[&str]) -> Result<Easing, ParseEasingError> {
    let mut raw = Vec::new();
    for argument in arguments {
        let mut parts = argument.split_whitespace();
        let output = parts
            .next()
            .and_then(number)
            .ok_or(ParseEasingError::InvalidArguments)?;
        let inputs: Vec<&str> = parts.collect();
        if inputs.len() > 2 {
            return Err(ParseEasingError::InvalidArguments);
        }
        if inputs.is_empty() {
            raw.push(RawStop {
                input: None,
                output,
            });
        }
        // A stop with two positions is two stops with the same output.
        for input in inputs {
            let percent = input
                .strip_suffix('%')
                .and_then(number)
                .ok_or(ParseEasingError::InvalidArguments)?;
            raw.push(RawStop {
                input: Some(percent / 100.0),
                output,
            });
        }
    }
    if raw.len() < 2 {
        return Err(ParseEasingError::InvalidArguments);
    }
    Ok(Easing::from_stops(resolve_inputs(raw)))
}

/// The CSS rules for the inputs of `linear()`: the first defaults to 0 and the last to 1, an input
/// is never below an earlier one, and missing ones are spread evenly between their neighbors.
fn resolve_inputs(mut raw: Vec<RawStop>) -> Vec<Stop> {
    let last = raw.len() - 1;
    raw[0].input.get_or_insert(0.0);
    raw[last].input.get_or_insert(1.0);
    let mut highest = f64::NEG_INFINITY;
    for stop in &mut raw {
        if let Some(input) = stop.input {
            highest = highest.max(input);
            stop.input = Some(highest);
        }
    }
    let mut index = 1;
    while index < raw.len() {
        if raw[index].input.is_some() {
            index += 1;
            continue;
        }
        // The run of stops without an input, from `index` up to the next one that has it.
        let end = (index..raw.len())
            .find(|&i| raw[i].input.is_some())
            .unwrap_or(last);
        let from = raw[index - 1].input.unwrap_or(0.0);
        let to = raw[end].input.unwrap_or(1.0);
        let missing = f64::from(u32::try_from(end - index + 1).unwrap_or(u32::MAX));
        let gap = (to - from) / missing;
        for (offset, stop) in raw[index..end].iter_mut().enumerate() {
            let position = f64::from(u32::try_from(offset + 1).unwrap_or(u32::MAX));
            stop.input = Some(from + gap * position);
        }
        index = end;
    }
    raw.into_iter()
        .map(|stop| Stop {
            input: stop.input.unwrap_or(0.0),
            output: stop.output,
        })
        .collect()
}

#[cfg(test)]
#[path = "_parse.test.rs"]
mod tests;
