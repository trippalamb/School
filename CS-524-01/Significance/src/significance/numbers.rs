use std::fmt;
use std::ops::{Add, Sub, Mul, Div};

/// Trait for numbers that track significance/error
pub trait Number: Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Clone + fmt::Display + Sized {
    fn value(&self) -> f64;
    fn error(&self) -> f64;
    fn with_error(value: f64, error: f64) -> Self;
    
    fn new(value: f64) -> Self {
        Self::with_error(value, 0.0)
    }
}

/// Real number type that tracks significance
#[derive(Debug, Clone, PartialEq)]
pub struct Real {
    value: f64,
    error: f64,
}

impl Real {
    pub fn new(value: f64) -> Self {
        Self { value, error: 0.0 }
    }
    
    pub fn with_error(value: f64, error: f64) -> Self {
        Self { value: value, error: error.abs() }
    }
}

impl Number for Real {
    fn value(&self) -> f64 {
        self.value
    }
    
    fn error(&self) -> f64 {
        self.error
    }
    
    fn with_error(value: f64, error: f64) -> Self {
        Self::with_error(value, error)
    }
}

impl fmt::Display for Real {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.error == 0.0 {
            write!(f, "{}", self.value)
        } else {
            write!(f, "{} ± {}", self.value, self.error)
        }
    }
}

impl Add for Real {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        // For independent uncertainties, combine in quadrature
        let combined_error = (self.error.powi(2) + other.error.powi(2)).sqrt();
        Self::with_error(self.value + other.value, combined_error)
    }
}

impl Sub for Real {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        // For independent uncertainties, combine in quadrature
        let combined_error = (self.error.powi(2) + other.error.powi(2)).sqrt();
        Self::with_error(self.value - other.value, combined_error)
    }
}

impl Mul for Real {
    type Output = Self;
    
    fn mul(self, other: Self) -> Self {
        let result_value = self.value * other.value;
        
        // Handle edge cases where value is zero
        let result_error = if self.value == 0.0 || other.value == 0.0 {
            // If either operand is exactly zero, result is zero with error from the other operand
            if self.value == 0.0 && other.value == 0.0 {
                0.0 // Both zero: result has no uncertainty
            } else if self.value == 0.0 {
                self.error * other.value.abs()
            } else {
                other.error * self.value.abs()
            }
        } else {
            // Standard relative uncertainty propagation for multiplication
            let rel_error_self = self.error / self.value.abs();
            let rel_error_other = other.error / other.value.abs();
            let combined_rel_error = (rel_error_self.powi(2) + rel_error_other.powi(2)).sqrt();
            combined_rel_error * result_value.abs()
        };
        
        Self::with_error(result_value, result_error)
    }
}

impl Div for Real {
    type Output = Self;
    
    fn div(self, other: Self) -> Self {
        if other.value == 0.0 {
            // Division by zero - return infinity with infinite error
            let result_value = self.value / other.value; // This will be ±∞ or NaN
            Self::with_error(result_value, f64::INFINITY)
        } else {
            let result_value = self.value / other.value;
            
            // Relative uncertainty propagation for division
            let rel_error_self = if self.value == 0.0 { 0.0 } else { self.error / self.value.abs() };
            let rel_error_other = other.error / other.value.abs();
            let combined_rel_error = (rel_error_self.powi(2) + rel_error_other.powi(2)).sqrt();
            let result_error = combined_rel_error * result_value.abs();
            
            Self::with_error(result_value, result_error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.000001;

    #[test]
    fn test_real_addition() {
        let x = Real::new(5.5);
        let y = Real::new(3.2);
        let result = x + y;
        assert!((result.value() - 8.7).abs() < EPSILON);
        assert!((result.error() - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_real_addition_w_uncertainty() {
        let x = Real::with_error(5.5, 0.2);
        let y = Real::with_error(3.2, 0.05);
        let result = x + y;
        assert!((result.value() - 8.7 ).abs() < EPSILON);
        assert!((result.error() - 0.25).abs() < EPSILON);
    }

    #[test]
    fn test_real_subtraction() {
        let z = Real::new(10.7);
        let y = Real::new(4.3);
        let result = z - y;
        assert!((result.value() - 6.4).abs() < EPSILON);
        assert!((result.error() - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_real_subtraction_w_uncertainty() {
        let z = Real::with_error(10.7, 0.08);
        let y = Real::with_error(4.3, 0.12);
        let result = z - y;
        assert!((result.value() - 6.4).abs() < EPSILON);
        assert!((result.error() - 0.2).abs() < EPSILON);
    }

    #[test]
    fn test_real_multiplication() {
        let z = Real::new(5.0);
        let y = Real::new(2.3);
        let result = z * y;
        assert!((result.value() - 11.5).abs() < EPSILON);
        assert!((result.error() - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_real_multiplication_w_uncertainty() {
        let z = Real::with_error(5.0, 0.2);
        let y = Real::with_error(2.3, 0.05);
        let result = z * y;
        assert!((result.value() - 11.5).abs() < EPSILON);
        assert!((result.error() - 0.5235456045).abs() < EPSILON);
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
        assert_eq!(format!("{}", with_error), "3.14 ± 0.01");
    }

    // Zero tests
    #[test]
    fn test_multiplication_with_zero() {
        let zero = Real::new(0.0);
        let x = Real::with_error(5.0, 0.1);
        let result = zero * x;
        assert_eq!(result.value(), 0.0);
        assert_eq!(result.error(), 0.0);
    }

    #[test]
    fn test_multiplication_zero_with_uncertainty() {
        let zero_with_error = Real::with_error(0.0, 0.1);
        let x = Real::with_error(5.0, 0.2);
        let result = zero_with_error * x;
        assert_eq!(result.value(), 0.0);
        assert_eq!(result.error(), 0.5); // 0.1 * |5.0| = 0.5
    }

    #[test]
    fn test_both_zero_multiplication() {
        let zero1 = Real::with_error(0.0, 0.1);
        let zero2 = Real::with_error(0.0, 0.05);
        let result = zero1 * zero2;
        assert_eq!(result.value(), 0.0);
        assert_eq!(result.error(), 0.0);
    }

    // Basic division tests
    #[test]
    fn test_real_division() {
        let x = Real::new(10.0);
        let y = Real::new(2.0);
        let result = x / y;
        assert!((result.value() - 5.0).abs() < EPSILON);
        assert!((result.error() - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_real_division_w_uncertainty() {
        let x = Real::with_error(10.0, 0.1);
        let y = Real::with_error(2.0, 0.02);
        let result = x / y;
        assert!((result.value() - 5.0).abs() < EPSILON);
        // Relative errors: 0.1/10.0 = 0.01, 0.02/2.0 = 0.01
        // Combined: sqrt(0.01² + 0.01²) = sqrt(0.0002) ≈ 0.01414
        // Result error: 0.01414 * 5.0 ≈ 0.0707
        assert!((result.error() - 0.070710678).abs() < 0.001);
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
        let result = zero / x;
        assert_eq!(result.value(), 0.0);
        assert_eq!(result.error(), 0.0);
    }

    #[test]
    fn test_zero_with_error_division() {
        let zero_with_error = Real::with_error(0.0, 0.1);
        let x = Real::new(4.0);
        let result = zero_with_error / x;
        assert_eq!(result.value(), 0.0);
        assert_eq!(result.error(), 0.025); // 0.1 / |4.0| = 0.025
    }

    #[test]
    fn test_division_large_uncertainty() {
        let x = Real::with_error(1.0, 0.5);
        let y = Real::with_error(0.5, 0.25);
        let result = x / y;
        assert!((result.value() - 2.0).abs() < EPSILON);
        // Relative errors: 0.5/1.0 = 0.5, 0.25/0.5 = 0.5
        // Combined: sqrt(0.5² + 0.5²) = sqrt(0.5) ≈ 0.7071
        // Result error: 0.7071 * 2.0 ≈ 1.4142
        assert!((result.error() - 1.414213562).abs() < 0.001);
    }
}