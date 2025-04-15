/// Converts Celsius to Fahrenheit.
///
/// # Examples
/// ```
/// use p22::temperature::celsius2fahrenheit;
///
/// let fahrenheit = celsius2fahrenheit(0);
/// assert_eq!(fahrenheit, 32);
/// ```
///
pub fn celsius2fahrenheit(celsius: i32) -> i32 {
    (celsius * 9 / 5) + 32
}

/// Converts Fahrenheit to Celsius.
///
/// # Examples
///
/// ```
/// use p22::temperature::fahrenheit2celsius;
///
/// let celsius = fahrenheit2celsius(0);
/// assert_eq!(celsius, -17);
/// ```
pub fn fahrenheit2celsius(fahrenheit: i32) -> i32 {
    (fahrenheit - 32) * 5 / 9
}

/// Converts Fahrenheit to Celsius precisely.
///
/// # Examples
///
/// ```
/// use p22::temperature::fahrenheit2celsius_precise;
///
/// let celsius_precise = fahrenheit2celsius_precise(0);
/// assert_eq!(celsius_precise, -17.777779);
/// ```
pub fn fahrenheit2celsius_precise(fahrenheit: i32) -> f32 {
    let float_fahrenheit = fahrenheit as f32;
    (float_fahrenheit - 32.0) * 5.0 / 9.0
}

#[cfg(test)]
mod c2f_tests {
    use super::*;

    #[test]
    fn test_zero_celsius2fahrenheit() {
        let result = celsius2fahrenheit(0);
        assert_eq!(result, 32);
    }

    #[test]
    fn test_negative_celsius2fahrenheit() {
        let result = celsius2fahrenheit(-10);
        assert_eq!(result, 14);
    }

    #[test]
    fn test_positive_celsius2fahrenheit() {
        let result = celsius2fahrenheit(10);
        assert_eq!(result, 50);
    }
}

#[cfg(test)]
mod f2c_tests {
    use super::*;

    #[test]
    fn test_zero_fahrenheit2celsius() {
        let result = fahrenheit2celsius(0);
        assert_eq!(result, -17);
    }

    #[test]
    fn test_negative_fahrenheit2celsius() {
        let result = fahrenheit2celsius(-10);
        assert_eq!(result, -23);
    }

    #[test]
    fn test_positive_fahrenheit2celsius() {
        let result = fahrenheit2celsius(10);
        assert_eq!(result, -12);
    }
}

#[cfg(test)]
mod f2c_precise_tests {
    use super::*;

    #[test]
    fn test_zero_fahrenheit2celsius_precise() {
        let result = fahrenheit2celsius_precise(0);
        assert_eq!(result, -17.777779);
    }

    #[test]
    fn test_negative_fahrenheit2celsius_precise() {
        let result = fahrenheit2celsius_precise(-10);
        assert_eq!(result, -23.333334);
    }

    #[test]
    fn test_positive_fahrenheit2celsius_precise() {
        let result = fahrenheit2celsius_precise(10);
        assert_eq!(result, -12.222222);
    }
}
