use std::io::stdin;

fn main() {
    println!("Fahrenheit to Celsius Convertor!");

    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    let celsius: f32 = input.trim().parse().expect("Please enter a valid number");
    let fahrenheit: f32 = input.trim().parse().expect("Please enter a valid number");

    let cel_far: f32 = celsius_to_fahrenheit(celsius);
    let far_cel: f32 = fahrenheit_to_celsius(fahrenheit);

    println!("{celsius}°C = {cel_far}°F");
    println!("{fahrenheit}°F = {far_cel}°C");
}

fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
