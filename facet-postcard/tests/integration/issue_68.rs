//! Regression test for https://github.com/facet-rs/facet-format/issues/68
//!
//! A `facet_value::Value` holding a float with an integral value (`1.0`) was
//! serialized as an integer: JSON wrote `1`, and tagged formats such as
//! postcard emitted the `I64` dynamic value tag instead of `F64`.

use facet::Facet;
use facet_postcard::to_vec_with_shape;
use facet_testhelpers::test;
use facet_value::{VNumber, Value};

fn is_float(value: &Value) -> bool {
    value.as_number().unwrap().is_float()
}

#[test]
fn test_issue_68_json_integral_float_stays_float() {
    let v: Value = facet_json::from_str("1.0").unwrap();
    assert!(is_float(&v));
    assert_eq!(facet_json::to_string(&v).unwrap(), "1.0");
}

#[test]
fn test_issue_68_json_array_keeps_number_kinds() {
    let v: Value = facet_json::from_str("[1, 1.0, 1e3, -0.0, 2.5]").unwrap();
    let json = facet_json::to_string(&v).unwrap();
    assert_eq!(json, "[1,1.0,1000.0,-0.0,2.5]");

    let back: Value = facet_json::from_str(&json).unwrap();
    let kinds: Vec<bool> = back.as_array().unwrap().iter().map(is_float).collect();
    assert_eq!(kinds, [false, true, true, true, true]);
}

#[test]
fn test_issue_68_json_integers_unchanged() {
    for json in ["0", "42", "-17", "18446744073709551615"] {
        let v: Value = facet_json::from_str(json).unwrap();
        assert!(!is_float(&v));
        assert_eq!(facet_json::to_string(&v).unwrap(), json);
    }
}

#[test]
fn test_issue_68_postcard_integral_float_roundtrips_as_float() {
    let v: Value = VNumber::from_f64(1.0).into();
    let bytes = facet_postcard::to_vec(&v).unwrap();
    let back: Value = facet_postcard::from_slice(&bytes).unwrap();
    assert!(is_float(&back));
    assert_eq!(back, v);

    let int: Value = VNumber::from_i64(1).into();
    let back: Value = facet_postcard::from_slice(&facet_postcard::to_vec(&int).unwrap()).unwrap();
    assert!(!is_float(&back));
}

#[derive(Debug, Facet, PartialEq)]
struct Ints {
    signed: i32,
    unsigned: u64,
    wide_signed: i128,
    wide_unsigned: u128,
}

/// Integer-typed targets keep accepting integral floats from a dynamic value,
/// even though they are no longer reported through the integer getters.
#[test]
fn test_issue_68_with_shape_integral_floats_fill_integer_fields() {
    let value: Value = facet_json::from_str(
        r#"{"signed": -10.0, "unsigned": 20.0, "wide_signed": -30.0, "wide_unsigned": 40.0}"#,
    )
    .unwrap();
    let bytes = to_vec_with_shape(&value, Ints::SHAPE).unwrap();
    let ints: Ints = facet_postcard::from_slice(&bytes).unwrap();
    assert_eq!(
        ints,
        Ints {
            signed: -10,
            unsigned: 20,
            wide_signed: -30,
            wide_unsigned: 40,
        }
    );
}

#[test]
fn test_issue_68_with_shape_rejects_floats_that_are_not_integers() {
    for json in [
        r#"{"signed": 1.5, "unsigned": 0, "wide_signed": 0, "wide_unsigned": 0}"#,
        r#"{"signed": 0, "unsigned": -1.0, "wide_signed": 0, "wide_unsigned": 0}"#,
        r#"{"signed": 0, "unsigned": 1e30, "wide_signed": 0, "wide_unsigned": 0}"#,
        r#"{"signed": 0, "unsigned": 0, "wide_signed": 1e30, "wide_unsigned": 0}"#,
    ] {
        let value: Value = facet_json::from_str(json).unwrap();
        assert!(
            to_vec_with_shape(&value, Ints::SHAPE).is_err(),
            "expected {json} to be rejected"
        );
    }
}

#[derive(Debug, Facet, PartialEq)]
struct Typed {
    x: f64,
    y: f32,
}

/// The "related" point of the issue: typed float fields must be written the
/// same way with and without facet-json's `fast` feature.
#[test]
fn test_issue_68_json_typed_floats_keep_fraction() {
    let json = facet_json::to_string(&Typed { x: 1.0, y: 2.0 }).unwrap();
    assert_eq!(json, r#"{"x":1.0,"y":2.0}"#);
    assert_eq!(facet_json::to_string(&-0.0f64).unwrap(), "-0.0");
    assert_eq!(facet_json::to_string(&0.1f64).unwrap(), "0.1");

    let back: Typed = facet_json::from_str(&json).unwrap();
    assert_eq!(back, Typed { x: 1.0, y: 2.0 });
}
