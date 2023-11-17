use std::io;

// Step 1: Create an enum called Operation
#[derive(Debug)]
enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

// Step 2 and 3: Create a function called calculate
fn calculate(operation: &Operation) -> f64 {
    match operation {
        Operation::Add(x, y) => x + y,
        Operation::Subtract(x, y) => x - y,
        Operation::Multiply(x, y) => x * y,
        Operation::Divide(x, y) => {
            if *y != 0.0 {
                *x / *y
            } else {
                println!("Error: Division by zero!");
                std::f64::NAN
            }
        }
    }
}

fn main() {
    // Step 4: Prompt the user for input
    println!("Enter the first number:");
    let first_number = get_user_input();

    println!("Enter the operation (+, -, *, /):");
    let operation_str = get_user_input();

    println!("Enter the second number:");
    let second_number = get_user_input();

    // Step 5: Parse user input into appropriate variables
    let first_number: f64 = first_number.trim().parse().expect("Invalid input");
    let second_number: f64 = second_number.trim().parse().expect("Invalid input");

    // Step 6: Create an Operation enum instance
    let operation = match operation_str.trim() {
        "+" => Operation::Add(first_number, second_number),
        "-" => Operation::Subtract(first_number, second_number),
        "*" => Operation::Multiply(first_number, second_number),
        "/" => Operation::Divide(first_number, second_number),
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    // Step 7 and 8: Call the calculate function and print the result
    let result = calculate(&operation);
    println!("Result: {}", result);
}

// Helper function to get user input
fn get_user_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input
}