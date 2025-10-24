use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// Trait for numbers that track significance/error
pub trait Number: 
    Add<Output = Self> + Sub<Output = Self> + 
    Mul<Output = Self> + Div<Output = Self> + 
    Rem<Output = Self> + Neg<Output = Self> +
    Clone + fmt::Display + Sized {
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
    
    pub fn power(&self, other: Self) -> Self {
        return Self{value:self.value.powf(other.value), error: 0.0};
    }

    pub fn root(&self, other: Self) -> Self {
        return Self{value:self.value.powf(1.0/other.value), error: 0.0};
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

impl Rem for Real {
    type Output = Self;

    fn rem(self, other: Self) -> Self {
        let combined_error = 0.0;
        Self::with_error(self.value % other.value, combined_error)
    }
}

impl Neg for Real {
    type Output = Self;
    
    fn neg(self) -> Self {
        Self::with_error(-self.value, self.error)
    }
}

pub fn assert_real(actual: &Real, expected: Real) {
    
    const EPSILON: f64 = 0.000001;
    
    assert!(
        (actual.value() - expected.value()).abs() < EPSILON,
        "Value mismatch: expected {}, got {}",
        expected.value(),
        actual.value()
    );
    assert!(
        (actual.error() - expected.error()).abs() < EPSILON,
        "Error mismatch: expected {}, got {}",
        expected.error(),
        actual.error()
    );
}

