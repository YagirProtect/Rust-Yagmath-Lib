use crate::conversion::{ConversionErrorKind, ConversionMode, RoundingMode};
use crate::vector::{Vector2, Vector3, Vector4};

#[test]
fn every_numeric_source_and_target_pair_supports_checked_and_clamp() {
    macro_rules! check_targets {
        ($source:ty; $($target:ty),+) => {$(
            for mode in [ConversionMode::Checked, ConversionMode::Clamp] {
                let a = Vector2::new(1 as $source, 2 as $source);
                assert_eq!(a.convert::<$target>(mode).unwrap(), Vector2::new(1 as $target, 2 as $target));
                let a = Vector3::new(1 as $source, 2 as $source, 3 as $source);
                assert_eq!(a.convert::<$target>(mode).unwrap(), Vector3::new(1 as $target, 2 as $target, 3 as $target));
                let a = Vector4::new(1 as $source, 2 as $source, 3 as $source, 4 as $source);
                assert_eq!(a.convert::<$target>(mode).unwrap(), Vector4::new(1 as $target, 2 as $target, 3 as $target, 4 as $target));
            }
        )+};
    }
    macro_rules! check_sources {
        ($($source:ty),+) => {$(
            check_targets!($source; i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
        )+};
    }
    check_sources!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

#[test]
fn integer_identity_preserves_exact_extrema_in_all_modes() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let v = Vector4::new(<$type>::MIN, <$type>::MAX, 0, 1);
            for mode in [ConversionMode::Checked, ConversionMode::Clamp, ConversionMode::Wrap] {
                assert_eq!(v.convert::<$type>(mode).unwrap(), v);
            }
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    );
}

#[test]
fn wrap_supports_every_integer_pair() {
    macro_rules! check_targets {
        ($source:ty; $($target:ty),+) => {$(
            let v = Vector2::new(1 as $source, 42 as $source);
            assert_eq!(v.convert::<$target>(ConversionMode::Wrap).unwrap(),
                Vector2::new(1 as $target, 42 as $target));
        )+};
    }
    macro_rules! check_sources {
        ($($source:ty),+) => {$(
            check_targets!($source; i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
        )+};
    }
    check_sources!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    );
}

#[test]
fn narrowing_integer_modes_produce_explicit_results() {
    let v = Vector2::new(300_i32, -130);
    assert_eq!(
        v.convert::<i8>(ConversionMode::Clamp).unwrap(),
        Vector2::new(127, -128)
    );
    assert_eq!(
        v.convert::<i8>(ConversionMode::Wrap).unwrap(),
        Vector2::new(44, 126)
    );
    let error = v.convert::<i8>(ConversionMode::Checked).unwrap_err();
    assert_eq!(error.component, "x");
    assert_eq!(error.kind, ConversionErrorKind::OutOfRange);
    assert_eq!(v, Vector2::new(300, -130));

    let v = Vector2::new(-1_i32, 256);
    assert_eq!(
        v.convert::<u8>(ConversionMode::Clamp).unwrap(),
        Vector2::new(0, 255)
    );
    assert_eq!(
        v.convert::<u8>(ConversionMode::Wrap).unwrap(),
        Vector2::new(255, 0)
    );
}

#[test]
fn exhaustive_i16_to_i8_conversion_respects_range_and_wrap() {
    for value in i16::MIN..=i16::MAX {
        let v = Vector2::new(value, 0);
        assert_eq!(
            v.convert::<i8>(ConversionMode::Clamp).unwrap().x,
            value.clamp(-128, 127) as i8
        );
        assert_eq!(
            v.convert::<i8>(ConversionMode::Wrap).unwrap().x,
            value as i8
        );
        let result = v.convert::<i8>(ConversionMode::Checked);
        if (-128..=127).contains(&value) {
            assert_eq!(result.unwrap().x as i16, value);
        } else {
            assert_eq!(result.unwrap_err().kind, ConversionErrorKind::OutOfRange);
        }
    }
}

#[test]
fn wide_integer_conversions_do_not_pass_through_float() {
    let v = Vector2::new(u128::MAX, i128::MAX as u128 + 1);
    assert_eq!(
        v.convert::<i128>(ConversionMode::Clamp).unwrap(),
        Vector2::new(i128::MAX, i128::MAX)
    );
    assert_eq!(
        v.convert::<i128>(ConversionMode::Wrap).unwrap(),
        Vector2::new(-1, i128::MIN)
    );
    assert_eq!(
        v.convert::<i128>(ConversionMode::Checked).unwrap_err().kind,
        ConversionErrorKind::OutOfRange
    );
    let v = Vector2::new(i128::MIN, -1);
    assert_eq!(
        v.convert::<u128>(ConversionMode::Clamp).unwrap(),
        Vector2::new(0, 0)
    );
    assert_eq!(
        v.convert::<u128>(ConversionMode::Wrap).unwrap(),
        Vector2::new(1_u128 << 127, u128::MAX)
    );
    assert_eq!(
        Vector2::new(i128::MAX - 1, i128::MAX)
            .convert::<i128>(ConversionMode::Checked)
            .unwrap(),
        Vector2::new(i128::MAX - 1, i128::MAX)
    );
    assert_eq!(
        Vector2::new(u64::MAX - 1, u64::MAX)
            .convert::<i128>(ConversionMode::Checked)
            .unwrap(),
        Vector2::new(u64::MAX as i128 - 1, u64::MAX as i128)
    );
}

#[test]
fn float_to_integer_uses_explicit_rounding_modes() {
    let v = Vector4::new(1.9_f64, -1.9, 2.5, -2.5);
    let mode = ConversionMode::Checked;
    assert_eq!(v.convert::<i32>(mode).unwrap(), Vector4::new(1, -1, 2, -2));
    assert_eq!(
        v.convert_with_rounding::<i32>(mode, RoundingMode::Truncate)
            .unwrap(),
        Vector4::new(1, -1, 2, -2)
    );
    assert_eq!(
        v.convert_with_rounding::<i32>(mode, RoundingMode::Round)
            .unwrap(),
        Vector4::new(2, -2, 3, -3)
    );
    assert_eq!(
        v.convert_with_rounding::<i32>(mode, RoundingMode::Floor)
            .unwrap(),
        Vector4::new(1, -2, 2, -3)
    );
    assert_eq!(
        v.convert_with_rounding::<i32>(mode, RoundingMode::Ceil)
            .unwrap(),
        Vector4::new(2, -1, 3, -2)
    );
    assert_eq!(
        Vector2::new(1.9_f32, 2.5)
            .convert_with_rounding::<u8>(mode, RoundingMode::Round)
            .unwrap(),
        Vector2::new(2, 3)
    );
}

#[test]
fn rounding_happens_before_range_checks() {
    let v = Vector2::new(127.5_f64, -128.5);
    assert_eq!(
        v.convert::<i8>(ConversionMode::Checked).unwrap(),
        Vector2::new(127, -128)
    );
    assert_eq!(
        v.convert_with_rounding::<i8>(ConversionMode::Checked, RoundingMode::Round)
            .unwrap_err()
            .kind,
        ConversionErrorKind::OutOfRange
    );
    assert_eq!(
        v.convert_with_rounding::<i8>(ConversionMode::Clamp, RoundingMode::Round)
            .unwrap(),
        Vector2::new(127, -128)
    );
    let v = Vector2::new(-0.5_f64, 255.5);
    assert_eq!(
        v.convert::<u8>(ConversionMode::Checked).unwrap(),
        Vector2::new(0, 255)
    );
    assert!(
        v.convert_with_rounding::<u8>(ConversionMode::Checked, RoundingMode::Round)
            .is_err()
    );
    assert_eq!(
        v.convert_with_rounding::<u8>(ConversionMode::Clamp, RoundingMode::Round)
            .unwrap(),
        Vector2::new(0, 255)
    );
}

#[test]
fn float_to_integer_checks_exclusive_upper_bounds_for_every_target() {
    macro_rules! check_types {
        ($($type:ty),+) => {$(
            let bits = <$type>::BITS as i32;
            let unsigned = <$type>::MIN == 0;
            let upper = 2.0_f64.powi(if unsigned { bits } else { bits - 1 });
            let below_upper = f64::from_bits(upper.to_bits() - 1);
            assert!(Vector2::new(upper, 0.0).convert::<$type>(ConversionMode::Checked).is_err());
            assert!(Vector2::new(below_upper, 0.0).convert::<$type>(ConversionMode::Checked).is_ok());
            let lower = if unsigned { 0.0 } else { -upper };
            assert!(Vector2::new(lower, 0.0).convert::<$type>(ConversionMode::Checked).is_ok());
            let below_lower = if unsigned { -1.0 } else { f64::from_bits(lower.to_bits() + 1) };
            assert!(Vector2::new(below_lower, 0.0)
                .convert_with_rounding::<$type>(ConversionMode::Checked, RoundingMode::Floor).is_err());
        )+};
    }
    check_types!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    );
}

#[test]
fn checked_integer_to_float_detects_precision_loss_at_mantissa_boundaries() {
    assert!(
        Vector2::new(1_u32 << 24, (1_u32 << 24) + 2)
            .convert::<f32>(ConversionMode::Checked)
            .is_ok()
    );
    assert_eq!(
        Vector2::new((1_u32 << 24) + 1, 0)
            .convert::<f32>(ConversionMode::Checked)
            .unwrap_err()
            .kind,
        ConversionErrorKind::PrecisionLoss
    );
    assert!(
        Vector2::new(1_u64 << 53, (1_u64 << 53) + 2)
            .convert::<f64>(ConversionMode::Checked)
            .is_ok()
    );
    assert_eq!(
        Vector2::new((1_u64 << 53) + 1, 0)
            .convert::<f64>(ConversionMode::Checked)
            .unwrap_err()
            .kind,
        ConversionErrorKind::PrecisionLoss
    );
    assert_eq!(
        Vector2::new(i64::MAX, 0)
            .convert::<f64>(ConversionMode::Checked)
            .unwrap_err()
            .kind,
        ConversionErrorKind::PrecisionLoss
    );
    assert_eq!(
        Vector2::new(u128::MAX, 0)
            .convert::<f64>(ConversionMode::Checked)
            .unwrap_err()
            .kind,
        ConversionErrorKind::PrecisionLoss
    );
    assert_eq!(
        Vector2::new(i128::MIN, 0)
            .convert::<f32>(ConversionMode::Checked)
            .unwrap()
            .x,
        -2.0_f32.powi(127)
    );
    assert_eq!(
        Vector2::new(u128::MAX, 0)
            .convert::<f32>(ConversionMode::Checked)
            .unwrap_err()
            .kind,
        ConversionErrorKind::OutOfRange
    );
    assert_eq!(
        Vector2::new(u128::MAX, 0)
            .convert::<f32>(ConversionMode::Clamp)
            .unwrap()
            .x,
        f32::MAX
    );
}

#[test]
fn checked_float_narrowing_detects_rounding_underflow_and_overflow() {
    assert_eq!(
        Vector2::new(1.5_f64, -2.5)
            .convert::<f32>(ConversionMode::Checked)
            .unwrap(),
        Vector2::new(1.5, -2.5)
    );
    assert_eq!(
        Vector2::new(0.1_f64, 0.0)
            .convert::<f32>(ConversionMode::Checked)
            .unwrap_err()
            .kind,
        ConversionErrorKind::PrecisionLoss
    );
    assert_eq!(
        Vector2::new(1e-300_f64, 0.0)
            .convert::<f32>(ConversionMode::Checked)
            .unwrap_err()
            .kind,
        ConversionErrorKind::PrecisionLoss
    );
    assert_eq!(
        Vector2::new(f64::MAX, 0.0)
            .convert::<f32>(ConversionMode::Checked)
            .unwrap_err()
            .kind,
        ConversionErrorKind::OutOfRange
    );
    assert_eq!(
        Vector2::new(f64::MAX, f64::MIN)
            .convert::<f32>(ConversionMode::Clamp)
            .unwrap(),
        Vector2::new(f32::MAX, f32::MIN)
    );
    let v = Vector2::new(f32::from_bits(1), -0.0_f32)
        .convert::<f64>(ConversionMode::Checked)
        .unwrap();
    assert_eq!(v.x, f64::from(f32::from_bits(1)));
    assert_eq!(v.y.to_bits(), (-0.0_f64).to_bits());
}

#[test]
fn non_finite_values_have_explicit_mode_dependent_behavior() {
    let v = Vector2::new(f64::INFINITY, f64::NEG_INFINITY);
    assert_eq!(
        v.convert::<i8>(ConversionMode::Clamp).unwrap(),
        Vector2::new(i8::MAX, i8::MIN)
    );
    assert_eq!(
        v.convert::<u8>(ConversionMode::Clamp).unwrap(),
        Vector2::new(u8::MAX, 0)
    );
    assert_eq!(
        v.convert::<f32>(ConversionMode::Clamp).unwrap(),
        Vector2::new(f32::MAX, f32::MIN)
    );
    assert_eq!(
        v.convert::<i64>(ConversionMode::Checked).unwrap_err().kind,
        ConversionErrorKind::NonFinite
    );
    assert_eq!(
        v.convert::<f64>(ConversionMode::Checked).unwrap_err().kind,
        ConversionErrorKind::NonFinite
    );
    for mode in [ConversionMode::Clamp, ConversionMode::Checked] {
        assert_eq!(
            Vector2::new(f64::NAN, 0.0)
                .convert::<i32>(mode)
                .unwrap_err()
                .kind,
            ConversionErrorKind::NonFinite
        );
        assert_eq!(
            Vector2::new(f64::NAN, 0.0)
                .convert::<f64>(mode)
                .unwrap_err()
                .kind,
            ConversionErrorKind::NonFinite
        );
    }
}

#[test]
fn wrap_rejects_conversions_involving_floats() {
    assert_eq!(
        Vector2::new(1.0_f32, 2.0)
            .convert::<i32>(ConversionMode::Wrap)
            .unwrap_err()
            .kind,
        ConversionErrorKind::UnsupportedMode
    );
    assert_eq!(
        Vector2::new(1_i32, 2)
            .convert::<f64>(ConversionMode::Wrap)
            .unwrap_err()
            .kind,
        ConversionErrorKind::UnsupportedMode
    );
    assert_eq!(
        Vector2::new(1.0_f64, 2.0)
            .convert::<f32>(ConversionMode::Wrap)
            .unwrap_err()
            .kind,
        ConversionErrorKind::UnsupportedMode
    );
}

#[test]
fn errors_identify_the_first_failing_component_without_changing_source() {
    let a = Vector2::new(1_i32, 300);
    assert_eq!(
        a.convert::<i8>(ConversionMode::Checked)
            .unwrap_err()
            .component,
        "y"
    );
    assert_eq!(a, Vector2::new(1, 300));
    let b = Vector3::new(1_i32, 2, 300);
    assert_eq!(
        b.convert::<i8>(ConversionMode::Checked)
            .unwrap_err()
            .component,
        "z"
    );
    let c = Vector4::new(1_i32, 2, 3, 300);
    let error = c.convert::<i8>(ConversionMode::Checked).unwrap_err();
    assert_eq!(error.component, "w");
    assert_eq!(
        error.to_string(),
        "component w: value is outside the target range"
    );
    let _: &dyn std::error::Error = &error;
    assert_eq!(c, Vector4::new(1, 2, 3, 300));
}
