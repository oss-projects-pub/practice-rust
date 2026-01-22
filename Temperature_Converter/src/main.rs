//! # Temperature Converter
//!
//! A simple command-line utility to convert temperatures from Celsius to Fahrenheit.
//! This project is part of the Rust practice lab.

use std::io;

/// The main entry point of the program.
///
/// It prompts the user for a Celsius value, reads the input from stdin,
/// parses it into a 32-bit float, and prints the result of the conversion.
fn main() {
    println!("Please enter a Celsius value:");

    let mut input = String::new();

    // Read the line from standard input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Trim the whitespace (including newline) and attempt to parse as f32
    let parse_result: Result<f32, _> = input.trim().parse();

    match parse_result {
        Ok(celsius_val) => {
            let fahrenheit_val = convert_celsius_to_fahrenheit(celsius_val);

            println!("The result of the calculation is: {:.1}°F", fahrenheit_val);
        }
        Err(err) => eprintln!("Error parsing string to float: {err}"),
    }
}

/// Converts a temperature value from Celsius to Fahrenheit.
///
/// # Arguments
///
/// * `celsius` - A 32-bit float representing the temperature in Celsius.
///
/// # Returns
///
/// * A 32-bit float representing the temperature in Fahrenheit.
///
/// # Formula
///
/// `(Celsius * 1.8) + 32.0`
fn convert_celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 1.8) + 32.0
}
