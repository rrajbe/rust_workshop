mod fibonacci;
mod temperature;

fn main() {
    println!("25°C = {}°F", temperature::celsius2fahrenheit(25));
    println!("25°F = {}°C", temperature::fahrenheit2celsius(25));
    println!(
        "25°F = {}°C Precisely",
        temperature::fahrenheit2celsius_precise(25)
    );

    println!("Fibonacci_rec(10) = {}", fibonacci::fibonacci_rec(10));
    println!("Fibonacci_loop(10) = {}", fibonacci::fibonacci_loop(10));
}
