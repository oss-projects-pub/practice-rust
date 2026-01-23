use std::io;

/// The entry point of the program.
/// It interactively asks for an index and prints the corresponding Fibonacci number.
fn main() {
    println!("Enter the index (n) to calculate the nth Fibonacci number:");

    let mut user_input = String::new();

    // Read input from the terminal
    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input");

    // Parse the input string into an integer (i32)
    let n_index: Result<i32, _> = user_input.trim().parse();

    match n_index {
        Ok(n) => {
            // Validate that index is not negative for this implementation
            if n < 0 {
                eprintln!("Error: Please enter a non-negative integer.");
                return;
            }

            // Calculate the Fibonacci number at index n
            let fibonacci_value = fibonacci(n);

            println!("Fibonacci({n}) is: {fibonacci_value}");
        }
        Err(e) => eprintln!("Invalid input: Please enter an integer. (Detail: {e})"),
    }
}

/// Computes the nth Fibonacci number using tree recursion.
///
/// The Fibonacci sequence follows the recurrence relation:
/// F(0) = 0
/// F(1) = 1
/// F(n) = F(n-1) + F(n-2)
///
/// # Arguments
/// * `n` - The position in the sequence (0-indexed).
///
/// # Returns
/// * The Fibonacci value at position `n`.
fn fibonacci(n: i32) -> i32 {
    // Base Case: F(0) = 0, F(1) = 1
    if n <= 1 {
        n
    } else {
        // Recursive Step: Sum the two preceding Fibonacci numbers
        let sum = fibonacci(n - 1) + fibonacci(n - 2);

        sum
    }
}
