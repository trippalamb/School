use std::fmt;
use std::ops::{Add, Sub};

/// Trait for numbers that track significance/error
pub trait Number: Add<Output = Self> + Sub<Output = Self> + Clone + fmt::Display + Sized {
    /// Get the nominal value
    fn value(&self) -> f64;
    
    /// Get the absolute error/uncertainty
    fn error(&self) -> f64;
    
    /// Create a new number with specified value and error
    fn with_error(value: f64, error: f64) -> Self;
    
    /// Create a new number with zero error
    fn new(value: f64) -> Self {
        Self::with_error(value, 0.0)
    }
}

/// Integer type that tracks significance
#[derive(Debug, Clone, PartialEq)]
pub struct Integer {
    value: i64,
    error: f64,
}

impl Integer {
    pub fn new(value: i64) -> Self {
        Self { value, error: 0.0 }
    }
    
    pub fn with_error(value: i64, error: f64) -> Self {
        Self { value, error }
    }
}

impl Number for Integer {
    fn value(&self) -> f64 {
        self.value as f64
    }
    
    fn error(&self) -> f64 {
        self.error
    }
    
    fn with_error(value: f64, error: f64) -> Self {
        Self::with_error(value as i64, error)
    }
}

impl fmt::Display for Integer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.error == 0.0 {
            write!(f, "{}", self.value)
        } else {
            write!(f, "{} ± {}", self.value, self.error)
        }
    }
}

impl Add for Integer {
    type Output = Self;
    
    fn add(self, _other: Self) -> Self {
        // TODO: Implement proper addition with error propagation
        unimplemented!("Addition not yet implemented")
    }
}

impl Sub for Integer {
    type Output = Self;
    
    fn sub(self, _other: Self) -> Self {
        // TODO: Implement proper subtraction with error propagation
        unimplemented!("Subtraction not yet implemented")
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
        Self { value, error }
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
    
    fn add(self, _other: Self) -> Self {
        // TODO: Implement proper addition with error propagation
        unimplemented!("Addition not yet implemented")
    }
}

impl Sub for Real {
    type Output = Self;
    
    fn sub(self, _other: Self) -> Self {
        // TODO: Implement proper subtraction with error propagation
        unimplemented!("Subtraction not yet implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Addition not yet implemented")]
    fn test_integer_addition() {
        let x = Integer::new(5);
        let y = Integer::new(3);
        let _result = x + y; // This should panic since addition isn't implemented
    }

    #[test]
    #[should_panic(expected = "Subtraction not yet implemented")]
    fn test_integer_subtraction() {
        let z = Integer::new(10);
        let y = Integer::new(4);
        let _result = z - y; // This should panic since subtraction isn't implemented
    }

    #[test]
    #[should_panic(expected = "Addition not yet implemented")]
    fn test_real_addition() {
        let x = Real::new(5.5);
        let y = Real::new(3.2);
        let _result = x + y; // This should panic since addition isn't implemented
    }

    #[test]
    #[should_panic(expected = "Subtraction not yet implemented")]
    fn test_real_subtraction() {
        let z = Real::new(10.7);
        let y = Real::new(4.3);
        let _result = z - y; // This should panic since subtraction isn't implemented
    }

    #[test]
    fn test_integer_with_error() {
        let num = Integer::with_error(42, 0.5);
        assert_eq!(num.value(), 42.0);
        assert_eq!(num.error(), 0.5);
    }

    #[test]
    fn test_real_with_error() {
        let num = Real::with_error(3.14159, 0.001);
        assert_eq!(num.value(), 3.14159);
        assert_eq!(num.error(), 0.001);
    }

    #[test]
    fn test_number_display() {
        let exact = Integer::new(42);
        let with_error = Integer::with_error(42, 0.5);
        
        assert_eq!(format!("{}", exact), "42");
        assert_eq!(format!("{}", with_error), "42 ± 0.5");
        
        let real_exact = Real::new(3.14);
        let real_with_error = Real::with_error(3.14, 0.01);
        
        assert_eq!(format!("{}", real_exact), "3.14");
        assert_eq!(format!("{}", real_with_error), "3.14 ± 0.01");
    }
}