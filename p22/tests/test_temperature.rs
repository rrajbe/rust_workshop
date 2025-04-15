use p22::temperature;

#[test]
fn test_zero_celsius2fahrenheit() {
    let result = temperature::celsius2fahrenheit(0);
    assert_eq!(result, 32);
}

#[test]
fn test_negative_celsius2fahrenheit() {
    let result = temperature::celsius2fahrenheit(-10);
    assert_eq!(result, 14);
}

#[test]
fn test_positive_celsius2fahrenheit() {
    let result = temperature::celsius2fahrenheit(10);
    assert_eq!(result, 50);
}

#[test]
fn test_zero_fahrenheit2celsius() {
    let result = temperature::fahrenheit2celsius(0);
    assert_eq!(result, -17);
}

#[test]
fn test_negative_fahrenheit2celsius() {
    let result = temperature::fahrenheit2celsius(-10);
    assert_eq!(result, -23);
}

#[test]
fn test_positive_fahrenheit2celsius() {
    let result = temperature::fahrenheit2celsius(10);
    assert_eq!(result, -12);
}

#[test]
fn test_zero_fahrenheit2celsius_precise() {
    let result = temperature::fahrenheit2celsius_precise(0);
    assert_eq!(result, -17.777779);
}

#[test]
fn test_negative_fahrenheit2celsius_precise() {
    let result = temperature::fahrenheit2celsius_precise(-10);
    assert_eq!(result, -23.333334);
}

#[test]
fn test_positive_fahrenheit2celsius_precise() {
    let result = temperature::fahrenheit2celsius_precise(10);
    assert_eq!(result, -12.222222);
}
