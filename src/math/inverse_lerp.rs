use super::Math;
use crate::extensions::to_f64::t_to_f64::ToF64;

impl Math {
    /// Returns the position of value in start..end, clamped to [0, 1].
    /// Equal bounds return 0, including bounds that become equal after conversion.
    /// Reversed ranges are allowed. NaN can propagate; infinite values clamp to
    /// an endpoint when the calculation is defined. No input causes a panic.
    /// Use checked_inverse_lerp to reject non-finite or degenerate inputs.
    pub fn inverse_lerp<T: ToF64>(start: T, end: T, value: T) -> f64 {
        Self::inverse_lerp_unclamped(start, end, value).clamp(0.0, 1.0)
    }

    /// Returns (value - start) / (end - start), without clamping.
    /// Equal bounds return 0. Non-finite inputs can produce NaN/infinity;
    /// an unrepresentably large result is infinite. No input causes a panic.
    /// Wide integer inputs may lose precision during conversion to f64.
    pub fn inverse_lerp_unclamped<T: ToF64>(start: T, end: T, value: T) -> f64 {
        let start = start.to_f64();
        let end = end.to_f64();
        let value = value.to_f64();
        if start == end {
            return 0.0;
        }
        calculate(start, end, value)
    }

    /// Returns the position of value in start..end, clamped to [0, 1].
    /// Reversed ranges are allowed. Returns None for equal/non-finite bounds
    /// or non-finite values. Inputs are converted to f64 before calculation.
    pub fn checked_inverse_lerp<T: ToF64>(start: T, end: T, value: T) -> Option<f64> {
        let (start, end, value) = checked_inputs(start, end, value)?;

        if start < end {
            if value <= start {
                return Some(0.0);
            }
            if value >= end {
                return Some(1.0);
            }
        } else {
            if value >= start {
                return Some(0.0);
            }
            if value <= end {
                return Some(1.0);
            }
        }

        let t = calculate(start, end, value);
        t.is_finite().then(|| t.clamp(0.0, 1.0))
    }

    /// Returns (value - start) / (end - start), without clamping.
    /// Reversed ranges are allowed. Returns None for equal/non-finite bounds,
    /// non-finite values or a result outside f64's finite range.
    /// Wide integers may lose precision when converted to f64.
    pub fn checked_inverse_lerp_unclamped<T: ToF64>(start: T, end: T, value: T) -> Option<f64> {
        let (start, end, value) = checked_inputs(start, end, value)?;
        let t = calculate(start, end, value);
        t.is_finite().then_some(t)
    }
}

/// Converts one inverse-lerp input triple to f64 and validates it.
/// Rejects non-finite inputs and endpoints that become equal after conversion;
/// returns converted values so checked callers do not repeat conversion.
fn checked_inputs<T: ToF64>(start: T, end: T, value: T) -> Option<(f64, f64, f64)> {
    let start = start.to_f64();
    let end = end.to_f64();
    let value = value.to_f64();
    if !start.is_finite() || !end.is_finite() || !value.is_finite() || start == end {
        return None;
    }
    Some((start, end, value))
}

/// Evaluates an unclamped inverse-lerp ratio after endpoint equality handling.
/// Ordinary callers handle equal bounds first; checked callers reject them.
/// Preserve IEEE behavior for non-finite input, exact endpoints for finite input,
/// and halve finite differences when subtraction would overflow.
fn calculate(start: f64, end: f64, value: f64) -> f64 {
    if !start.is_finite() || !end.is_finite() || !value.is_finite() {
        return (value - start) / (end - start);
    }
    if value == start {
        return 0.0;
    }
    if value == end {
        return 1.0;
    }

    let offset = value - start;
    let range = end - start;
    if offset.is_finite() && range.is_finite() {
        offset / range
    } else {
        // Halving before subtracting prevents intermediate overflow for huge inputs.
        (value * 0.5 - start * 0.5) / (end * 0.5 - start * 0.5)
    }
}
