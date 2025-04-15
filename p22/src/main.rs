mod temperature;

fn main() {
    println!("25°C = {}°F", temperature::celsius2fahrenheit(25));
    println!("25°F = {}°C", temperature::fahrenheit2celsius(25));
    println!(
        "25°F = {}°C Precisely",
        temperature::fahrenheit2celsius_precise(25)
    );
}
