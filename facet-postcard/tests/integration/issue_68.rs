//! Regression test for https://github.com/facet-rs/facet-format/issues/68
//!
//! A `facet_value::Value` holding a float with an integral value (`1.0`) was
//! serialized as an integer: JSON wrote `1`, and tagged formats such as
//! postcard emitted the `I64` dynamic value tag instead of `F64`.

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
