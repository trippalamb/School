//------------------
// (Tripp) Milton Lamb
// Fall 2025, Nov 29 2025
// CS-524: Programming Languages
// Final Project
//------------------

use crate::significance::numbers::{Number, Real, assert_real};

#[test]
fn test_real_addition() {
    let x = Real::new(5.5);
    let y = Real::new(3.2);
    let actual = x + y;
    let expected = Real::new(8.7);
    assert_real(&actual, expected);
}

#[test]
fn test_real_addition_w_uncertainty() {
    let x = Real::with_error(5.5, 0.2);
    let y = Real::with_error(3.2, 0.05);
    let actual = x + y;
    let expected = Real::with_error(8.7, 0.20615528128088306);
    assert_real(&actual, expected);
}

#[test]
fn test_real_subtraction() {
    let z = Real::new(10.7);
    let y = Real::new(4.3);
    let actual = z - y;
    let expected = Real::new(6.4);
    assert_real(&actual, expected);
}

#[test]
fn test_real_subtraction_w_uncertainty() {
    let z = Real::with_error(10.7, 0.08);
    let y = Real::with_error(4.3, 0.12);
    let actual = z - y;
    let expected = Real::with_error(6.4, 0.14422205101855956);
    assert_real(&actual, expected);
}

#[test]
fn test_real_multiplication() {
    let z = Real::new(5.0);
    let y = Real::new(2.3);
    let actual = z * y;
    let expected = Real::new(11.5);
    assert_real(&actual, expected);
}

#[test]
fn test_real_multiplication_w_uncertainty() {
    let z = Real::with_error(5.0, 0.2);
    let y = Real::with_error(2.3, 0.05);
    let actual = z * y;
    let expected = Real::with_error(11.5, 0.5235456045);
    assert_real(&actual, expected);
}

#[test]
fn test_real_with_error() {
    let num = Real::with_error(3.14159, 0.001);
    assert_eq!(num.value(), 3.14159);
    assert_eq!(num.error(), 0.001);
}

#[test]
fn test_number_display() {
    let exact = Real::new(3.14);
    let with_error = Real::with_error(3.14, 0.01);
    
    assert_eq!(format!("{}", exact), "3.14");
    assert_eq!(format!("{}", with_error), "3.14 +/- 0.01");
}

// Zero tests
#[test]
fn test_multiplication_with_zero() {
    let zero = Real::new(0.0);
    let x = Real::with_error(5.0, 0.1);
    let actual = zero * x;
    let expected = Real::new(0.0);
    assert_real(&actual, expected);
}

#[test]
fn test_multiplication_zero_with_uncertainty() {
    let zero_with_error = Real::with_error(0.0, 0.1);
    let x = Real::with_error(5.0, 0.2);
    let actual = zero_with_error * x;
    let expected = Real::with_error(0.0, 0.5);
    assert_real(&actual, expected);
}

#[test]
fn test_both_zero_multiplication() {
    let zero1 = Real::with_error(0.0, 0.1);
    let zero2 = Real::with_error(0.0, 0.05);
    let actual = zero1 * zero2;
    let expected = Real::new(0.0);
    assert_real(&actual, expected);
}

// Basic division tests
#[test]
fn test_real_division() {
    let x = Real::new(10.0);
    let y = Real::new(2.0);
    let actual = x / y;
    let expected = Real::new(5.0);
    assert_real(&actual, expected);
}

#[test]
fn test_real_division_w_uncertainty() {
    let x = Real::with_error(10.0, 0.1);
    let y = Real::with_error(2.0, 0.02);
    let actual = x / y;
    let expected = Real::with_error(5.0, 0.070710678);
    assert_real(&actual, expected);
}

#[test]
fn test_division_by_zero() {
    let x = Real::new(5.0);
    let zero = Real::new(0.0);
    let result = x / zero;
    assert!(result.value().is_infinite());
    assert!(result.error().is_infinite());
}

#[test]
fn test_zero_division() {
    let zero = Real::new(0.0);
    let x = Real::new(5.0);
    let actual = zero / x;
    let expected = Real::new(0.0);
    assert_real(&actual, expected);
}

#[test]
fn test_division_large_uncertainty() {
    let x = Real::with_error(1.0, 0.5);
    let y = Real::with_error(0.5, 0.25);
    let actual = x / y;
    let expected = Real::with_error(2.0, 1.414213562);
    assert_real(&actual, expected);
}